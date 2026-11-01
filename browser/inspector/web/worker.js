import init, { inspect_snapshot } from './pkg/court_inspector.js';
await init();
postMessage({type:'ready'});
onmessage=({data})=>{try{postMessage({type:'result',id:data.id,text:inspect_snapshot(data.text)});}catch(error){postMessage({type:'error',id:data.id,message:String(error)});}};
