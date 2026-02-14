type YmLogEvent = {
  time: number;
  addr: string;
  data: string;
};

type LaneElements = {
  root: HTMLElement;
  track: HTMLElement;
};

export type LogVisualizer = {
  renderFromJson: (jsonText: string | null | undefined) => void;
  clear: () => void;
};

const DEFAULT_CHANNELS = 8;
const MIN_TRACK_WIDTH = 640;
const MAX_TRACK_WIDTH = 6400;
const PIXELS_PER_SECOND = 180;
const EVENT_WIDTH = 8;

function parseHexByte(value: string): number | null {
  const match = /^0x([0-9a-fA-F]{1,2})$/.exec(value.trim());
  if (!match) return null;
  const parsed = parseInt(match[1], 16);
  return Number.isNaN(parsed) ? null : parsed;
}

function detectChannel(addrHex: string, dataHex: string, channelCount: number): number | null {
  const addr = parseHexByte(addrHex);
  if (addr === null) return null;

  if (addr === 0x08) {
    const data = parseHexByte(dataHex);
    if (data !== null) {
      return (data >> 3) & 0x07;
    }
  }

  if (addr >= 0x20) {
    return addr & Math.max(channelCount - 1, 0);
  }

  return null;
}

function normalizeEvents(parsed: unknown): YmLogEvent[] {
  if (!parsed || typeof parsed !== 'object') return [];
  const rawEvents = (parsed as { events?: unknown }).events;
  if (!Array.isArray(rawEvents)) return [];

  return rawEvents
    .map(event => {
      if (!event || typeof event !== 'object') return null;
      const e = event as { time?: unknown; addr?: unknown; data?: unknown };
      const time =
        typeof e.time === 'number'
          ? e.time
          : typeof e.time === 'string'
            ? Number(e.time)
            : Number.NaN;
      const addr = typeof e.addr === 'string' ? e.addr : '';
      const data = typeof e.data === 'string' ? e.data : '';
      if (!Number.isFinite(time) || !addr || !data) return null;
      return { time, addr, data };
    })
    .filter((e): e is YmLogEvent => Boolean(e));
}

function laneColor(index: number | null): string {
  if (index === null) return '#8a8a8a';
  const hue = (index * 37) % 360;
  return `hsl(${hue}, 70%, 55%)`;
}

function createLane(label: string, trackWidth: number): LaneElements {
  const root = document.createElement('div');
  root.className = 'log-visualizer-lane';

  const labelEl = document.createElement('div');
  labelEl.className = 'log-visualizer-label';
  labelEl.textContent = label;
  root.appendChild(labelEl);

  const track = document.createElement('div');
  track.className = 'log-visualizer-track';
  track.style.width = `${trackWidth}px`;
  root.appendChild(track);

  return { root, track };
}

function computeTrackWidth(events: YmLogEvent[]): number {
  const maxTime = events.reduce((max, e) => Math.max(max, e.time), 0);
  const width = maxTime * PIXELS_PER_SECOND + 40;
  return Math.min(MAX_TRACK_WIDTH, Math.max(MIN_TRACK_WIDTH, width));
}

export function createLogVisualizer(
  container: HTMLElement | null,
  options?: { channelCount?: number },
): LogVisualizer {
  if (!container) {
    return {
      renderFromJson: () => {
        /* no-op */
      },
      clear: () => {
        /* no-op */
      },
    };
  }

  const channelCount = Math.max(1, Math.min(16, options?.channelCount ?? DEFAULT_CHANNELS));

  const renderEmpty = (message: string) => {
    container.classList.add('log-visualizer', 'log-visualizer--empty');
    container.innerHTML = '';
    const empty = document.createElement('div');
    empty.className = 'log-visualizer-empty';
    empty.textContent = message;
    container.appendChild(empty);
  };

  const renderFromJson = (jsonText: string | null | undefined) => {
    if (!jsonText || jsonText.trim().length === 0) {
      renderEmpty('変換結果がまだありません。');
      return;
    }

    let events: YmLogEvent[] = [];
    try {
      const parsed = JSON.parse(jsonText);
      events = normalizeEvents(parsed);
    } catch {
      renderEmpty('ログ JSON を解釈できませんでした。');
      return;
    }

    if (events.length === 0) {
      renderEmpty('描画できるイベントがありません。');
      return;
    }

    const trackWidth = computeTrackWidth(events);
    container.classList.add('log-visualizer');
    container.classList.remove('log-visualizer--empty');
    container.innerHTML = '';

    const lanes: Record<string, LaneElements> = {};

    for (let ch = 0; ch < channelCount; ch += 1) {
      const lane = createLane(`CH ${ch}`, trackWidth);
      container.appendChild(lane.root);
      lanes[ch.toString()] = lane;
    }

    let globalLane: LaneElements | null = null;
    const ensureGlobalLane = () => {
      if (globalLane) return globalLane;
      globalLane = createLane('GLOBAL', trackWidth);
      container.appendChild(globalLane.root);
      return globalLane;
    };

    events.forEach((event, index) => {
      const channel = detectChannel(event.addr, event.data, channelCount);
      const lane =
        channel !== null && channel >= 0 && channel < channelCount
          ? lanes[channel.toString()]
          : ensureGlobalLane();
      const marker = document.createElement('div');
      marker.className = 'log-visualizer-event';
      marker.style.left = `${Math.max(0, Math.min(trackWidth - EVENT_WIDTH, event.time * PIXELS_PER_SECOND))}px`;
      marker.style.backgroundColor = laneColor(channel);
      marker.title = `t=${event.time.toFixed(3)}s addr=${event.addr} data=${event.data} (#${index})`;
      lane.track.appendChild(marker);
    });
  };

  renderEmpty('YM2151 ログを変換するとここに描画します。');

  return {
    renderFromJson,
    clear: () => renderEmpty('YM2151 ログを変換するとここに描画します。'),
  };
}
