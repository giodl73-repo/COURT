import {test,expect} from '@playwright/test';
import fs from 'node:fs';
const fixture=JSON.parse(fs.readFileSync(new URL('../fixtures/snapshot.json',import.meta.url),'utf8'));
test('authored example uses real worker and native behavior',async({page})=>{
 await page.goto('./'); await expect(page.locator('#status')).toContainText('Ready.');
 await page.getByRole('button',{name:'Load authored example'}).click();
 await expect(page.locator('#input')).toHaveValue(/court:authored-example/);
 await page.getByRole('button',{name:'Inspect snapshot',exact:true}).click();
 await expect(page.locator('#status')).toContainText('Snapshot inspected');
 const r=JSON.parse(await page.locator('#report').textContent());
 expect(r.available_commands).toEqual(['inspect','reset']);expect(r.present_roles).toEqual(['Surface','Control']);expect(r.unsupported_features[0].fallback).toBe('Text label');
});
test('own input, native rejection, clearing stale exports and keyboard',async({page})=>{
 await page.goto('./');await expect(page.locator('#status')).toContainText('Ready.');
 const v=structuredClone(fixture);v.actions[0].command='own command';v.experience.title='<img src=x onerror=alert(1)>';
 await page.locator('#input').fill(JSON.stringify(v));await page.getByRole('button',{name:'Inspect snapshot',exact:true}).focus();await page.keyboard.press('Enter');await expect(page.locator('#status')).toContainText('Snapshot inspected');
 expect(JSON.parse(await page.locator('#report').textContent()).available_commands[0]).toBe('own command');expect(await page.locator('img').count()).toBe(0);
 await page.locator('#input').fill('{"unknown":true}');await expect(page.locator('#download')).toBeDisabled();await page.locator('#inspect').click();await expect(page.locator('#status')).toContainText('Input was rejected');
});
test('native and browser exact JSON responses',async({page})=>{
 await page.goto('./');const cases=JSON.parse(fs.readFileSync('.cache/native-cases.json','utf8'));
 const results=await page.evaluate(async cases=>{const m=await import('./pkg/court_inspector.js');await m.default();return cases.map(c=>m.inspect_snapshot(c.input));},cases);
 expect(results).toEqual(cases.map(c=>c.output));
});
