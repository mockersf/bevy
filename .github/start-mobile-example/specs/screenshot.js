var assert = require('assert');
const percyScreenshot = require('@percy/appium-app');


describe('Running Bevy Example', () => {
  it('can take a screenshot', async () => {

    const current_package = await browser.getCurrentPackage();
    assert.equal(current_package, 'org.bevyengine.example');

    await browser.saveScreenshot('./screenshot.png');

    await percyScreenshot('Main Screen');

  });
});