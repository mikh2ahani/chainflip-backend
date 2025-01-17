import { encodeAddress } from '@polkadot/util-crypto';
import { Asset } from '@chainflip-io/cli';
import { newSwap } from './new_swap';
import { send, sendViaCfTester } from './send';
import { getBalance } from './get_balance';
import {
  getChainflipApi,
  observeBalanceIncrease,
  observeEvent,
  observeCcmReceived,
  assetToChain,
  observeSwapScheduled,
  observeSwapEvents,
  observeBroadcastSuccess,
} from '../shared/utils';
import { CcmDepositMetadata } from '../shared/new_swap';

function encodeDestinationAddress(address: string, destAsset: Asset): string {
  let destAddress = address;

  if (destAddress && destAsset === 'DOT') {
    destAddress = encodeAddress(destAddress);
  }

  return destAddress;
}

export type SwapParams = {
  sourceAsset: Asset;
  destAsset: Asset;
  depositAddress: string;
  destAddress: string;
  channelId: number;
};

export async function requestNewSwap(
  sourceAsset: Asset,
  destAsset: Asset,
  destAddress: string,
  tag = '',
  messageMetadata?: CcmDepositMetadata,
): Promise<SwapParams> {
  const chainflipApi = await getChainflipApi();

  const addressPromise = observeEvent(
    'swapping:SwapDepositAddressReady',
    chainflipApi,

    (event) => {
      // Find deposit address for the right swap by looking at destination address:
      const destAddressEvent = encodeDestinationAddress(
        event.data.destinationAddress[assetToChain(destAsset)],
        destAsset,
      );
      if (!destAddressEvent) return false;

      const destAssetMatches = event.data.destinationAsset.toUpperCase() === destAsset;
      const sourceAssetMatches = event.data.sourceAsset.toUpperCase() === sourceAsset;
      const destAddressMatches =
        destAddressEvent.toLowerCase() ===
        encodeDestinationAddress(destAddress, destAsset).toLowerCase();

      // CF Parameters is always set to '' by the SDK for now
      const ccmMetadataMatches = messageMetadata
        ? event.data.channelMetadata !== null &&
          event.data.channelMetadata.message === messageMetadata.message &&
          Number(event.data.channelMetadata.gasBudget.replace(/,/g, '')) ===
            messageMetadata.gasBudget
        : event.data.channelMetadata === null;

      return destAddressMatches && destAssetMatches && sourceAssetMatches && ccmMetadataMatches;
    },
  );
  await newSwap(sourceAsset, destAsset, destAddress, messageMetadata);

  const res = (await addressPromise).data;

  const depositAddress = res.depositAddress[assetToChain(sourceAsset)];
  const channelDestAddress = res.destinationAddress[assetToChain(destAsset)];
  const channelId = Number(res.channelId);

  console.log(`${tag} Swap address: ${depositAddress}`);
  console.log(`${tag} Destination address is: ${channelDestAddress} Channel ID is: ${channelId}`);

  return {
    sourceAsset,
    destAsset,
    depositAddress,
    destAddress,
    channelId,
  };
}

export enum SenderType {
  Address,
  Contract,
}

export async function doPerformSwap(
  { sourceAsset, destAsset, destAddress, depositAddress, channelId }: SwapParams,
  tag = '',
  messageMetadata?: CcmDepositMetadata,
  senderType = SenderType.Address,
  amount?: string,
) {
  const oldBalance = await getBalance(destAsset, destAddress);

  console.log(`${tag} Old balance: ${oldBalance}`);

  const swapScheduledHandle = observeSwapScheduled(sourceAsset, destAsset, channelId);

  const ccmEventEmitted = messageMetadata
    ? observeCcmReceived(sourceAsset, destAsset, destAddress, messageMetadata)
    : Promise.resolve();

  await (senderType === SenderType.Address
    ? send(sourceAsset, depositAddress, amount)
    : sendViaCfTester(sourceAsset, depositAddress));

  console.log(`${tag} Funded the address`);

  await swapScheduledHandle;

  console.log(`${tag} Waiting for balance to update`);

  try {
    const [newBalance] = await Promise.all([
      observeBalanceIncrease(destAsset, destAddress, oldBalance),
      ccmEventEmitted,
    ]);

    console.log(`${tag} Swap success! New balance: ${newBalance}!`);
  } catch (err) {
    throw new Error(`${tag} ${err}`);
  }
}

export async function performSwap(
  sourceAsset: Asset,
  destAsset: Asset,
  destAddress: string,
  swapTag?: string,
  messageMetadata?: CcmDepositMetadata,
  senderType = SenderType.Address,
  amount?: string,
) {
  const tag = swapTag ?? '';

  console.log(
    `${tag} The args are:  ${sourceAsset} ${destAsset} ${destAddress} ${
      messageMetadata ? `someMessage` : ''
    }`,
  );

  const swapParams = await requestNewSwap(
    sourceAsset,
    destAsset,
    destAddress,
    tag,
    messageMetadata,
  );
  await doPerformSwap(swapParams, tag, messageMetadata, senderType, amount);

  return swapParams;
}

// function to create a swap and track it until we detect the corresponding broadcast success
export async function performAndTrackSwap(
  sourceAsset: Asset,
  destAsset: Asset,
  destAddress: string,
  amount?: string,
  tag?: string,
) {
  const chainflipApi = await getChainflipApi();

  const swapParams = await requestNewSwap(sourceAsset, destAsset, destAddress, tag);

  await send(sourceAsset, swapParams.depositAddress, amount);
  console.log(`${tag} fund sent, waiting for the deposit to be witnessed..`);

  // SwapScheduled, SwapExecuted, SwapEgressScheduled, BatchBroadcastRequested
  const broadcastId = await observeSwapEvents(swapParams, chainflipApi, tag);

  if (broadcastId) await observeBroadcastSuccess(broadcastId);
  else throw new Error('Failed to retrieve broadcastId!');
  console.log(`${tag} broadcast executed succesfully, swap is complete!`);
}
