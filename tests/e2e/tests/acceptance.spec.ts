import { test, expect } from '@playwright/test';

test('index', async ({ page, browserName }) => {
  let user_agent = "Chrome";
  if (browserName === 'firefox') { user_agent = "Firefox" }
  if (browserName === 'webkit') { user_agent = "Safari" }

  await page.goto('https://ip.pdt.sh/');
  await expect(page.locator('#heading-project')).toContainText('ip.pdt.sh');
  //await page.goto('http://localhost:8000');
  //await expect(page.locator('#heading-project')).toContainText('ifconfig-rs');

  await expect(page.locator('#overview-user-agent')).toContainText(user_agent);
  await page.getByText('API').click();
  await expect(page.locator('#api-json-root')).toContainText(/"host":/);

  let regex = new RegExp('"browser": {\n.+"family": "' + user_agent + '"');
  await expect(page.locator('#api-json-root')).toContainText(regex);
});