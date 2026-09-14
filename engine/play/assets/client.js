'use strict';
const $ = id => document.getElementById(id);
const canvas = $('canvas'), ctx = canvas.getContext('2d');
let frame, target = null, item = null, busy = false, position = [480,450], walking = false;
const reduced = matchMedia('(prefers-reduced-motion: reduce)').matches;
async function command(name, value = '') {
  if (busy) return;
  busy = true;
  try {
    const response = await fetch('/api', { method:'POST', headers:{'Content-Type':'text/plain'}, body:name+'\n'+value });
    if (!response.ok) throw new Error('The local player could not answer.');
    const next = await response.json();
    if (frame && frame.room !== next.room) target = null;
    frame = next;
    position = frame.position;
    if (!frame.inventory.some(i => i.name === item)) item = null;
    if (frame.saved) {
      const blob = new Blob([frame.saved], {type:'text/plain'});
      const link = document.createElement('a');
      link.href = URL.createObjectURL(blob); link.download = 'patrol.save'; link.click();
      setTimeout(() => URL.revokeObjectURL(link.href), 1000);
    }
    render();
  } catch (error) { $('status').textContent = error.message; }
  finally { busy = false; }
  if (frame?.hold === 'walking' && !walking) animateWalk(frame.route);
}
function button(text, fn) { const b = document.createElement('button'); b.textContent = text; b.onclick = fn; return b; }
function render() {
  const idle = frame.hold === 'idle';
  $('room').textContent = frame.title;
  $('phase').textContent = frame.ended ? 'SHIFT COMPLETE' : frame.hold === 'walking' ? 'WALKING' : 'NIGHT SHIFT · PATROL';
  $('status').textContent = frame.message;
  $('ending').textContent = frame.ended ? 'The end. Load a save to play again.' : 'TAB to explore · ENTER to continue';
  $('save').disabled = !idle || frame.ended;
  $('load').disabled = !idle;
  $('speech').hidden = frame.hold !== 'speaking';
  $('line').textContent = frame.line;
  $('targets').replaceChildren();
  for (const t of frame.targets) {
    const b = button('', () => { if (!idle || frame.ended) return; target = t.name; render(); $('actions').querySelector('button')?.focus(); });
    b.className = 'target'; b.dataset.target = t.name;
    b.setAttribute('aria-label', t.label); b.disabled = !idle || !t.active || frame.ended;
    const [x,y,w,h] = t.rect;
    Object.assign(b.style, {left:x/9.6+'%',top:y/5.4+'%',width:w/9.6+'%',height:h/5.4+'%'});
    const label = document.createElement('span'); label.textContent = t.label; b.append(label); $('targets').append(b);
  }
  $('actions').replaceChildren();
  if (frame.conversation) target = null;
  const chosen = frame.targets.find(t => t.name === target);
  if (!chosen) target = null;
  $('context').textContent = frame.conversation ? 'Choose what to say.' : chosen ? chosen.label : item ? 'Choose a target for the item.' : 'Choose something in the room.';
  for (const e of frame.menu.filter(e => frame.conversation || (e.target === target && (e.object === null || e.object === item)))) {
    const b = button(e.text, () => command('choose',e.key)); b.dataset.choice = e.key; b.disabled = !idle || frame.ended; $('actions').append(b);
  }
  $('inventory').replaceChildren();
  if (!frame.inventory.length) { const p = document.createElement('p'); p.className = 'empty'; p.textContent = 'Empty pockets.'; $('inventory').append(p); }
  for (const entry of frame.inventory) {
    const b = button(entry.label + (entry.count > 1 ? ' × '+entry.count : ''), () => { item = item === entry.name ? null : entry.name; render(); });
    b.dataset.item = entry.name; b.setAttribute('aria-pressed',item === entry.name); b.disabled = !idle || frame.ended; $('inventory').append(b);
  }
  draw();
  if (frame.hold === 'speaking') $('dismiss').focus();
}
function polygon(points, color) { ctx.fillStyle=color; ctx.beginPath(); points.forEach(([x,y],i) => i ? ctx.lineTo(x,y) : ctx.moveTo(x,y)); ctx.closePath(); ctx.fill(); }
function box(x,y,w,h,color) { ctx.fillStyle=color; ctx.fillRect(x,y,w,h); }
function person(x,y,porter=false) {
  ctx.fillStyle='#07120d70'; ctx.beginPath(); ctx.ellipse(x,y+3,23,7,0,0,Math.PI*2); ctx.fill();
  box(x-13,y-64,26,42,porter?'#636d56':'#ae9971'); box(x-10,y-23,8,23,'#172522'); box(x+3,y-23,8,23,'#172522');
  ctx.fillStyle='#bdad86'; ctx.beginPath(); ctx.arc(x,y-76,11,0,Math.PI*2); ctx.fill(); box(x-13,y-87,26,7,porter?'#263b34':'#33352b');
  box(x-18,y-60,6,30,porter?'#636d56':'#ae9971'); box(x+12,y-60,6,30,porter?'#636d56':'#ae9971');
  if (porter) { box(x-17,y-51,36,22,'#bcc0a4'); box(x,y-51,1,22,'#647160'); }
}
function prop(t) {
  const [x,y,w,h]=t.rect;
  ctx.fillStyle='#08130f75'; ctx.beginPath(); ctx.ellipse(x+w/2,y+h+9,w*.6,12,0,0,Math.PI*2);ctx.fill();
  if (t.art === 'door') {
    box(x-7,y-7,w+14,h+7,'#0c1c19');box(x,y,w,h,'#30433b');box(x+8,y+8,w-16,h-16,'#25382f');
    box(x+w*.18,y+h*.12,w*.64,h*.36,'#536457');box(x+w*.18+5,y+h*.12+5,w*.64-10,h*.36-10,'#152724');
    for(let i=1;i<4;i++)box(x+w*.18+w*.16*i,y+h*.12,2,h*.36,'#536457');
    box(x+w-24,y+h*.64,11,4,'#c2a568');box(x+8,y+h-5,w-16,5,'#b7a474');
    if(t.name.includes('strongroom')||t.name.includes('steel')){ctx.strokeStyle='#829283';ctx.lineWidth=5;ctx.beginPath();ctx.arc(x+w*.5,y+h*.65,25,0,Math.PI*2);ctx.stroke();}
  } else if(t.art==='cabinet') {
    box(x,y,w,h,'#536057');box(x+7,y+7,w-14,h-14,'#34473e');
    for(let i=0;i<4;i++){box(x+16,y+18+i*49,w-32,42,'#4e5d4f');box(x+w*.42,y+30+i*49,w*.16,6,'#acaa88');}
  } else if(t.art==='person') { person(x+w/2,y+h,true); }
  else if(t.art==='kettle') {
    box(x-15,y+h+2,w+35,13,'#796f51');polygon([[x+12,y+h],[x+4,y+18],[x+w-22,y+18],[x+w-7,y+h]],'#a6ad8d');
    box(x+14,y+10,w-40,8,'#c7c5a4');ctx.strokeStyle='#b6ba99';ctx.lineWidth=8;ctx.beginPath();ctx.arc(x+w-12,y+45,21,-Math.PI/2,Math.PI/2);ctx.stroke();
  } else if(t.art==='switch') {box(x,y,w,h,'#596958');box(x+12,y+16,w-24,h-32,'#212f24');box(x+16,y+7,12,6,'#d4a558');}
  else if(t.art==='tin') {box(x,y,w,h,'#86885e');box(x-3,y,w+6,9,'#b6af79');box(x+w*.25,y+24,w*.5,2,'#54654e');}
  else { box(x,y,w,h,'#b3b596'); for(let i=0;i<4;i++)box(x+12,y+12+i*10,w-24,2,'#7b876c'); }
}
function draw() {
  if (!frame) return;
  const gradient=ctx.createLinearGradient(0,0,0,540);gradient.addColorStop(0,'#142823');gradient.addColorStop(.68,'#465346');gradient.addColorStop(1,'#172923');ctx.fillStyle=gradient;ctx.fillRect(0,0,960,540);
  box(0,310,960,9,'#172b23');box(0,318,960,6,'#68715a');
  for(let x=0;x<960;x+=80){box(x,324,2,70,'#263b2d');box(x+7,330,66,55,'#2b3e2d');}
  polygon(frame.floor,'#50604b');
  ctx.strokeStyle='#293d3080';ctx.lineWidth=1;for(let y=350;y<540;y+=32){ctx.beginPath();ctx.moveTo(0,y);ctx.lineTo(960,y);ctx.stroke();}for(let x=-600;x<1600;x+=85){ctx.beginPath();ctx.moveTo(480+(x-480)*.55,340);ctx.lineTo(x,540);ctx.stroke();}
  // One reusable drawing vocabulary, independent of hit regions and interaction guards.
  for(const t of frame.targets)prop(t);
  for(const f of frame.figures)person(...f.point,true);
  person(...position);
  const glow=ctx.createRadialGradient(480,80,5,480,80,400);glow.addColorStop(0,'#d6c88620');glow.addColorStop(1,'#00000000');ctx.fillStyle=glow;ctx.fillRect(0,0,960,540);
  box(443,52,74,5,'#172d25');box(451,57,58,7,'#c4c29a');
  ctx.strokeStyle='#0c1d1980';ctx.lineWidth=36;ctx.strokeRect(0,0,960,540);
}
async function animateWalk(route) {
  walking=true;
  const distances=route.slice(1).map((p,i)=>Math.hypot(p[0]-route[i][0],p[1]-route[i][1]));
  const total=distances.reduce((a,b)=>a+b,0), duration=reduced?0:total/240*1000;
  await new Promise(resolve=>{
    const start=performance.now();
    function tick(now){let remaining=duration?Math.min(1,(now-start)/duration)*total:total;let i=0;
      while(i<distances.length-1 && remaining>distances[i])remaining-=distances[i++];
      if(!distances.length)position=route[0];else {const t=distances[i]?Math.min(1,remaining/distances[i]):1;position=[route[i][0]+(route[i+1][0]-route[i][0])*t,route[i][1]+(route[i+1][1]-route[i][1])*t];}
      draw();if(now-start>=duration)resolve();else requestAnimationFrame(tick);
    }requestAnimationFrame(tick);
  });
  // A delayed tab still completes the same action once; no game clock runs underneath it.
  await command('arrive');walking=false;
}
canvas.onclick=event=>{if(!frame||busy||frame.hold!=='idle')return;const r=canvas.getBoundingClientRect();command('walk',Math.round((event.clientX-r.left)*960/r.width)+' '+Math.round((event.clientY-r.top)*540/r.height));};
$('dismiss').onclick=()=>command('dismiss');
$('save').onclick=()=>command('save');
$('load').onclick=()=>$('load-file').click();
$('load-file').onchange=async event=>{const file=event.target.files[0];if(file)await command('load',await file.text());event.target.value='';};
document.addEventListener('keydown',event=>{if(event.key==='Escape'&&frame?.hold==='idle'){target=null;item=null;render();}if(event.key===' '&&frame?.hold==='speaking'){event.preventDefault();command('dismiss');}});
command('frame');
