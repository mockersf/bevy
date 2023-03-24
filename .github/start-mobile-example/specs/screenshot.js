var assert = require('assert');
const percyScreenshot = require('@percy/appium-app');


describe('Running Bevy Example', () => {
  it('can take a screenshot', async () => {

    await new Promise(r => setTimeout(r, 2000));

    await browser.saveScreenshot('./screenshot.png');

    await percyScreenshot('Main Screen');

  });
});