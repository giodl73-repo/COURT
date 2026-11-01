const $=id=>document.getElementById(id), input=$('input');
let worker, generation=0, revision=0, timer, ready=false, report=null, pending=null;
const bytes=s=>new TextEncoder().encode(s).length;
function clearReport(){report=null;$('download').disabled=true;$('report').textContent='No report yet.';}
function fail(message){clearTimeout(timer);ready=false;pending=null;worker?.terminate();$('inspect').disabled=true;clearReport();$('status').textContent=message+' Restart engine to retry. Inputs are retained.';}
function start(){worker?.terminate();clearTimeout(timer);generation++;ready=false;pending=null;clearReport();$('inspect').disabled=true;$('status').textContent='Starting Rust engine…';const own=generation;worker=new Worker('./worker.js',{type:'module'});timer=setTimeout(()=>fail('Engine startup exceeded 12 seconds.'),12000);
worker.onerror=()=>{if(own===generation)fail('Rust engine failed.');};
worker.onmessage=({data})=>{if(own!==generation)return;if(data.type==='ready'){clearTimeout(timer);ready=true;$('inspect').disabled=false;$('status').textContent='Ready. Inspect your snapshot or load the example.';return;}if(!pending||data.id!==pending.id)return;clearTimeout(timer);const request=pending;pending=null;$('inspect').disabled=false;if(request.revision!==revision)return;if(data.type==='error'){fail(data.message);return;}report=data.text;$('report').textContent=report;$('download').disabled=false;$('status').textContent=JSON.parse(report).ok?'Snapshot inspected. No actions were executed.':'Input was rejected; see the report.';};}
input.addEventListener('input',()=>{revision++;clearReport();$('status').textContent='Input changed; previous report cleared.';});
$('inspect').onclick=()=>{clearReport();if(!ready)return;if(bytes(input.value)>524288){$('status').textContent='Input exceeds 512 KiB UTF-8; nothing was truncated.';return;}const id=crypto.randomUUID();pending={id,revision};$('inspect').disabled=true;$('status').textContent='Inspecting with Rust…';timer=setTimeout(()=>fail('Inspection exceeded 10 seconds.'),10000);worker.postMessage({id,text:input.value});};
$('restart').onclick=start;
$('example').onclick=async()=>{const own=revision;try{const response=await fetch('./snapshot.json');if(!response.ok)throw new Error('Example unavailable');const text=await response.text();if(revision!==own)return;input.value=text;revision++;clearReport();$('status').textContent='Authored example loaded. Inspect it or edit your own input.';}catch(e){$('status').textContent=String(e);}};
$('download').onclick=()=>{if(!report)return;const url=URL.createObjectURL(new Blob([report],{type:'application/json'}));const a=document.createElement('a');a.href=url;a.download='court-snapshot-report.json';a.click();setTimeout(()=>URL.revokeObjectURL(url),1000);};
start();
