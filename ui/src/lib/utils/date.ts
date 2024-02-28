import type { Record } from '@holochain/client';
import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import localizedFormat from 'dayjs/plugin/localizedFormat';
dayjs.extend(relativeTime);
dayjs.extend(localizedFormat);

export const formatTimestampLocal = (timestamp: number) => dayjs(timestamp).format('L LT');

export const formatTimestampHumanized = (timestamp: number) => dayjs(timestamp).fromNow();

export const formatRecordTimestampHumanized = (record: Record) => formatTimestampHumanized(record.signed_action.hashed.content.timestamp / 1000);
