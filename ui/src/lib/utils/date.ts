import type { Record } from '@holochain/client';
import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
dayjs.extend(relativeTime);

export const formatTimestampHumanized = (timestamp: number) => dayjs(timestamp).fromNow();

export const formatRecordTimestampHumanized = (record: Record) => formatTimestampHumanized(record.signed_action.hashed.content.timestamp / 1000);