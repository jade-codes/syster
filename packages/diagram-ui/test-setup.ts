import { Window } from "happy-dom";

// Set up happy-dom for React component testing
const window = new Window();
global.document = window.document as unknown as Document;
global.window = window as any;
global.navigator = window.navigator as unknown as Navigator;
global.HTMLElement = window.HTMLElement as unknown as typeof HTMLElement;
global.Element = window.Element as unknown as typeof Element;
global.SVGElement = window.SVGElement as unknown as typeof SVGElement;

// Mock ResizeObserver for React Flow
global.ResizeObserver = class ResizeObserver {
  observe() {}
  unobserve() {}
  disconnect() {}
};
