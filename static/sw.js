const cacheName = 'egui-template-pwa';
const filesToCache = [
  // Web Root
  './',
  // HTML Source
  './index.html',
  // CSS Source
  './styles.css',
  // JS Source
  './yew-app.js',
  // WebAssembly Source
  './yew-app_bg.wasm',
  // All icons
  'icons/android-chrome-96x96.png',
  'icons/browserconfig.xml',
  'icons/favicon-32x32.png',
  'icons/favicon.ico',
  'icons/safari-pinned-tab.svg',
  'icons/apple-touch-icon.png',
  'icons/favicon-16x16.png',
  'icons/favicon-384x384.png',
  'icons/mstile-150x150.png',
];

/* Start the service worker and cache all of the app's content */
self.addEventListener('install', function (e) {
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    }),
  );
});

/* Serve cached content when offline */
self.addEventListener('fetch', function (e) {
  e.respondWith(
    caches.match(e.request).then(function (response) {
      return response || fetch(e.request);
    }),
  );
});
