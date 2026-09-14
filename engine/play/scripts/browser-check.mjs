// Browser acceptance check. Start screen and headless Chrome with a debugging port first.
// Requires Node with built-in fetch and WebSocket; no packages or browser drivers.
// node browser-check.mjs [debug-port=9227] [game-url=http://127.0.0.1:8787] [screenshot=/tmp/klik-patrol.png]
import {writeFile} from 'node:fs/promises';
const [debugPort='9227',url='http://127.0.0.1:8787',screenshot='/tmp/klik-patrol.png']=process.argv.slice(2);
const targets=await (await fetch(`http://127.0.0.1:${debugPort}/json/list`)).json();
const target=targets.find(t=>t.type==='page');
if(!target)throw new Error('Chrome has no page');
const socket=new WebSocket(target.webSocketDebuggerUrl), pending=new Map(), errors=[];
let serial=0;
await new Promise((resolve,reject)=>{socket.onopen=resolve;socket.onerror=reject;});
socket.onmessage=event=>{
  const m=JSON.parse(event.data);
  if(m.id){const {resolve,reject}=pending.get(m.id);pending.delete(m.id);m.error?reject(new Error(m.error.message)):resolve(m.result);}
  if(m.method==='Runtime.exceptionThrown')errors.push(m.params.exceptionDetails.text);
};
function send(method,params={}){return new Promise((resolve,reject)=>{const id=++serial;pending.set(id,{resolve,reject});socket.send(JSON.stringify({id,method,params}));});}
async function evaluate(expression){const r=await send('Runtime.evaluate',{expression,awaitPromise:true,returnByValue:true});if(r.exceptionDetails)throw new Error(r.exceptionDetails.exception?.description||r.exceptionDetails.text);return r.result.value;}
async function wait(expression){return evaluate(`new Promise((resolve,reject)=>{const end=performance.now()+10000;function poll(){if(${expression})resolve(true);else if(performance.now()>end)reject(new Error('Timed out: '+${JSON.stringify(expression)}));else setTimeout(poll,20);}poll();})`);}
try {
  await send('Runtime.enable'); await send('Page.enable');
  await send('Emulation.setDeviceMetricsOverride',{width:1280,height:1000,deviceScaleFactor:1,mobile:false});
  await send('Emulation.setEmulatedMedia',{features:[{name:'prefers-reduced-motion',value:'reduce'}]});
  await send('Page.navigate',{url});
  await wait('typeof frame !== "undefined" && frame && !busy');
  if(await evaluate('frame.room')!=='lobby')throw new Error('Start a fresh server for this acceptance check');
  const image=await send('Page.captureScreenshot',{format:'png',captureBeyondViewport:true});
  await writeFile(screenshot,Buffer.from(image.data,'base64'));
  const initial=await evaluate(`fetch('/api',{method:'POST',body:'save\\n'}).then(r=>r.json()).then(r=>r.saved)`);
  if(!initial?.startsWith('klik-save 1 '))throw new Error('Save was not returned');
  const choices=['walk-to inner-door','walk-to canteen-door','pick-up tin','walk-to switch',
    'walk-to corridor-door','walk-to strongroom-door','use-with key on cabinet','pick-up bond-in-cabinet',
    'walk-to steel-door','walk-to lobby-door','walk-to street-doors'];
  for(const choice of choices){
    const entry=await evaluate(`frame.menu.find(e=>e.key===${JSON.stringify(choice)})`);
    if(!entry)throw new Error('Missing choice '+choice);
    if(entry.object)await evaluate(`document.querySelector('[data-item="'+${JSON.stringify(entry.object)}+'"]')?.click()`);
    await evaluate(`document.querySelector('[data-target="'+${JSON.stringify(entry.target)}+'"]')?.click()`);
    const available=await evaluate(`!!document.querySelector('[data-choice="'+${JSON.stringify(choice)}+'"]')`);
    if(!available)throw new Error('No visible action for '+choice);
    await evaluate(`document.querySelector('[data-choice="'+${JSON.stringify(choice)}+'"]')?.click()`);
    await wait('!busy && !walking && frame.hold !== "walking"');
    while(await evaluate('frame.hold === "speaking"')){
      if(!await evaluate('document.getElementById("save").disabled'))throw new Error('Save was enabled during speech');
      await evaluate('document.getElementById("dismiss").click()'); await wait('!busy');
    }
  }
  if(!await evaluate('frame.ended'))throw new Error('Winning route did not end');
  // Upload the saved contents through the same handler as the file input, then verify the boundary.
  await evaluate(`command('load',${JSON.stringify(initial)})`);await wait('!busy');
  if(await evaluate('frame.room')!=='lobby'||await evaluate('frame.ended'))throw new Error('Restore did not return to the initial boundary');
  await send('Emulation.setDeviceMetricsOverride',{width:390,height:844,deviceScaleFactor:1,mobile:true});
  await evaluate('new Promise(r=>requestAnimationFrame(()=>requestAnimationFrame(r)))');
  const overflow=await evaluate('document.documentElement.scrollWidth > innerWidth');
  if(overflow)throw new Error('Mobile layout overflows');
  const mobile=await send('Page.captureScreenshot',{format:'png',captureBeyondViewport:true});
  await writeFile(screenshot.replace(/\.png$/, '-mobile.png'),Buffer.from(mobile.data,'base64'));
  if(errors.length)throw new Error(errors.join('\n'));
  console.log('PASS: graphical winning route, response holds, save/restore, mobile layout, no browser exceptions.');
} finally {socket.close();}
