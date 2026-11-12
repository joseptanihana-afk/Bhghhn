import {createRoot} from 'react-dom/client';
import App from './App.tsx';
import './index.css';

createRoot(getElementByIdOrThrow('root')).render(<App />);

function getElementByIdOrThrow(id: string): HTMLElement {
  const el = document.getElementById(id);
  if (!el) throw new Error(`Root element #${id} not found`);
  return el;
}
