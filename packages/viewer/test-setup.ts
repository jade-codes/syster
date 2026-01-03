import { Window } from "happy-dom";

// Set up happy-dom for React component testing
const window = new Window();
global.document = window.document as any;
global.window = window as any;
global.navigator = window.navigator as any;
global.HTMLElement = window.HTMLElement as any;
global.Element = window.Element as any;
global.SVGElement = window.SVGElement as any;

// Mock ResizeObserver for React Flow
global.ResizeObserver = class ResizeObserver {
  observe() {}
  unobserve() {}
  disconnect() {}
};

// Mock DOMMatrixReadOnly for React Flow
global.DOMMatrixReadOnly = class DOMMatrixReadOnly {
  m22: number = 1;
  constructor() {}
} as any;
