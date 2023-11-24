const { log, device, by, element } = require('detox');

import { loadDarksideWallet } from "./e2e-utils/loadDarksideWallet.js";

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

describe('Syncs a darkside chain', () => {
  it('should sync the darkside chain', async () => {
    await loadDarksideWallet();
    await sleep(10000);

    // verify pool balances
    await waitFor(element(by.id('header.drawmenu'))).toBeVisible().withTimeout(sync_timeout);
    await element(by.id('header.drawmenu')).tap();
    await element(by.id('menu.syncreport')).tap();

    // // put the App to sleep because we need some progress in the syncing for 20 seconds
    // await sleep(20000);

    // await waitFor(async () => {
    //   const notyetsynced = Number(await element(by.id('syncreport.notyetsynced')).getAttributes('text'));
    //   return notyetsynced < 990000
    // }).withTimeout(sync_timeout);

    // // background for 5 seconds. The same effect that close & open the App.
    // await device.sendToHome();
    // await sleep(5000);
    // await device.launchApp({ newInstance: false });

    // // put the App to sleep because we need some progress in the syncing for 20 seconds
    // await sleep(20000);

    // await waitFor(async () => {
    //   const notyetsynced = Number(await element(by.id('syncreport.notyetsynced')).getAttributes('text'));
    //   return notyetsynced < 980000
    // }).withTimeout(sync_timeout);

    // // background for 5 seconds. The same effect that close & open the App.
    // await device.sendToHome();
    // await sleep(5000);
    // await device.launchApp({ newInstance: false });

    // // put the App to sleep because we need some progress in the syncing for 20 seconds
    // await sleep(20000);

    // await waitFor(async () => {
    //   const notyetsynced = Number(await element(by.id('syncreport.notyetsynced')).getAttributes('text'));
    //   return notyetsynced < 970000
    // }).withTimeout(sync_timeout);

    // // background for 5 seconds. The same effect that close & open the App.
    // await device.sendToHome();
    // await sleep(5000);
    // await device.launchApp({ newInstance: false });

    // // put the App to sleep because we need some progress in the syncing for 20 seconds
    // await sleep(20000);

    // await waitFor(async () => {
    //   const notyetsynced = Number(await element(by.id('syncreport.notyetsynced')).getAttributes('text'));
    //   return notyetsynced < 960000
    // }).withTimeout(sync_timeout);

    // // background for 5 seconds. The same effect that close & open the App.
    // await device.sendToHome();
    // await sleep(5000);
    // await device.launchApp({ newInstance: false });

    // // put the App to sleep because we need some progress in the syncing for 20 seconds
    // await sleep(20000);

    // await waitFor(async () => {
    //   const notyetsynced = Number(await element(by.id('syncreport.notyetsynced')).getAttributes('text'));
    //   return notyetsynced < 950000
    // }).withTimeout(sync_timeout);

    // // background for 5 seconds. The same effect that close & open the App.
    // await device.sendToHome();
    // await sleep(5000);
    // await device.launchApp({ newInstance: false });

    // // put the App to sleep because we need some progress in the syncing for 20 seconds
    // await sleep(20000);

    // await waitFor(async () => {
    //   const notyetsynced = Number(await element(by.id('syncreport.notyetsynced')).getAttributes('text'));
    //   return notyetsynced < 940000
    // }).withTimeout(sync_timeout);

    // // background for 5 seconds. The same effect that close & open the App.
    // await device.sendToHome();
    // await sleep(5000);
    // await device.launchApp({ newInstance: false });

    // // put the App to sleep because we need some progress in the syncing for 20 seconds
    // await sleep(20000);

    // await waitFor(async () => {
    //   const notyetsynced = Number(await element(by.id('syncreport.notyetsynced')).getAttributes('text'));
    //   return notyetsynced < 930000
    // }).withTimeout(sync_timeout);

    // // background for 5 seconds. The same effect that close & open the App.
    // await device.sendToHome();
    // await sleep(5000);
    // await device.launchApp({ newInstance: false });

    // // put the App to sleep because we need some progress in the syncing for 20 seconds
    // await sleep(20000);

    // await waitFor(async () => {
    //   const notyetsynced = Number(await element(by.id('syncreport.notyetsynced')).getAttributes('text'));
    //   return notyetsynced < 920000
    // }).withTimeout(sync_timeout);

    // // background for 5 seconds. The same effect that close & open the App.
    // await device.sendToHome();
    // await sleep(5000);
    // await device.launchApp({ newInstance: false });

    // // put the App to sleep because we need some progress in the syncing for 20 seconds
    // await sleep(20000);

    // await waitFor(async () => {
    //   const notyetsynced = Number(await element(by.id('syncreport.notyetsynced')).getAttributes('text'));
    //   return notyetsynced < 910000
    // }).withTimeout(sync_timeout);

    // // background for 5 seconds. The same effect that close & open the App.
    // await device.sendToHome();
    // await sleep(5000);
    // await device.launchApp({ newInstance: false });

    // // put the App to sleep because we need some progress in the syncing for 20 seconds
    // await sleep(20000);

    // // close sync report screen
    // await waitFor(element(by.id('syncreport.button.close'))).toBeVisible().withTimeout(sync_timeout);
    // await element(by.id('syncreport.button.close')).tap();

    // // we need to wait now for the fully sync green icon
    // await waitFor(element(by.id('header.sync-facheck'))).toBeVisible().withTimeout(sync_timeout);

    // // verify pool balances
    // await waitFor(element(by.id('header.drawmenu'))).toBeVisible().withTimeout(sync_timeout);
    // await element(by.id('header.drawmenu')).tap();
    // await element(by.id('menu.fund-pools')).tap();
    // await expect(element(by.id('orchard-total-balance.big-part'))).toHaveText(' 1.0000');
    // await expect(element(by.id('orchard-total-balance.small-part'))).not.toBeVisible();
    // await expect(element(by.id('orchard-spendable-balance.big-part'))).toHaveText(' 1.0000');
    // await expect(element(by.id('orchard-spendable-balance.small-part'))).not.toBeVisible();
    // await expect(element(by.id('sapling-total-balance.big-part'))).toHaveText(' 0.0000');
    // await expect(element(by.id('sapling-total-balance.small-part'))).not.toBeVisible();
    // await expect(element(by.id('sapling-spendable-balance.big-part'))).toHaveText(' 0.0000');
    // await expect(element(by.id('sapling-spendable-balance.small-part'))).not.toBeVisible();
    // await expect(element(by.id('transparent-balance.big-part'))).toHaveText(' 0.0000');
    // await expect(element(by.id('transparent-total-balance.small-part'))).not.toBeVisible();
    // await element(by.id('fund-pools.button.close')).tap();

    await sleep(100000);
  });
});
