import {defineConfig} from '@playwright/test';
const live=process.env.COURT_BASE_URL;
const site=process.env.COURT_SITE_DIR || '../../target/court-release';
export default defineConfig({testDir:'./tests',workers:1,retries:0,timeout:30000,
 use:{baseURL:live || 'http://127.0.0.1:4191/COURT/inspector/'},
 reporter:[['list'],['json',{outputFile:live?'.cache/live-browser-results.json':'.cache/browser-results.json'}]],
 webServer:live?undefined:{command:`python -m http.server 4191 --directory "${site}"`,url:'http://127.0.0.1:4191/COURT/inspector/',reuseExistingServer:false}
});
