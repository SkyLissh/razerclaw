<script lang="ts">
  import { m } from "$lib/paraglide/messages";

  import { Slider } from "$lib/components/ui/slider";
  import { formatNumber } from "$lib/utils";

  type Props = {
    pollRate: number;
    supportedPollRates: number[];
    onUpdatePollRate: (pollRate: number) => void;
  };

  const { pollRate, supportedPollRates, onUpdatePollRate }: Props = $props();

  let pollRateIndex = $derived.by(() => {
    if (!supportedPollRates) return 0;

    const index = supportedPollRates.indexOf(pollRate);
    return index >= 0 ? index : 0;
  });

  function updatePollRate() {
    const newPollRate = supportedPollRates[pollRateIndex];
    onUpdatePollRate(newPollRate);
  }
</script>

<div class="col-span-1 flex min-h-40 flex-col gap-6 rounded bg-card p-4">
  <div class="flex items-center justify-between">
    <p class="text-lg font-semibold">{m.poll_rate()}</p>
    <p class="text-2xl font-bold text-primary">{pollRate} Hz</p>
  </div>

  <div class="flex flex-col gap-2">
    <Slider
      type="single"
      bind:value={pollRateIndex}
      min={0}
      max={supportedPollRates.length - 1}
      tickLabels={supportedPollRates.map(formatNumber)}
      onValueCommit={updatePollRate}
    />
  </div>
</div>
