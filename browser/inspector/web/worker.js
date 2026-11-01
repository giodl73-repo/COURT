import init, { inspect_snapshot } from './pkg/court_inspector.js';
const started = performance.now();
await init();
postMessage({type: 'ready', initialization_ms: performance.now() - started});
onmessage = ({data}) => {
  try {
    const started = performance.now();
    const text = inspect_snapshot(data.text);
    postMessage({type: 'result', id: data.id, text, computation_ms: performance.now() - started});
  } catch (error) {
    postMessage({type: 'error', id: data.id, message: String(error)});
  }
};
