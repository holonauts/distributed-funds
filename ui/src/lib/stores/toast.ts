import { derived, writable } from 'svelte/store';
import { v4 as uuidv4 } from 'uuid';
import  sortBy from 'lodash/sortBy';

export enum ToastMessageType {
	Success = "Success",
	Error = "Error",
	Warning = "Warning",
	Info = "Info",
}

export interface ToastPayload {
	message_type: ToastMessageType;
	text: string;
	break_text?: boolean;
}

export type ToastData = ToastPayload & { timestamp: Date; id: string };

export type ToastDataStore = { [id: string]: ToastData };

const AUTO_HIDE_TOAST_DELAY = 5000;

function useToastsStore() {
  const toasts = writable<ToastDataStore>({});

  function add(payload: ToastPayload) {
    const id = uuidv4();
    const timestamp = new Date();
    toasts.update((val) => {
      val[id] = { ...payload, timestamp, id };
      return val;
    });

    // Only auto hide non-error toasts
    deleteAfterDelay(id);

    if(payload.message_type === ToastMessageType.Error) {
      console.error(payload);
    }
  }

  function deleteAfterDelay(id: string, delayMs: number = AUTO_HIDE_TOAST_DELAY) {
    setTimeout(() => {
      toasts.update((val) => {
        delete val[id];
        return val;
      });
    }, delayMs);
  }

  function error(text: string, payload: ToastPayload | object = {}) {
    add({
      message_type: ToastMessageType.Error,
      text,
      ...payload
    });
  }

  function success(text: string, payload: ToastPayload | object = {}) {
    add({
      message_type: ToastMessageType.Success,
      text,
      ...payload
    });
  }

  function info(text: string, payload: ToastPayload | object = {}) {
    add({
      message_type: ToastMessageType.Info,
      text,
      ...payload
    });
  }

  return {
    subscribe: toasts.subscribe,
    add,
    error,
    success,
    info
  }
}

export const toasts = useToastsStore();

export const toastsList = derived(toasts, (toasts) => sortBy(Object.values(toasts), [(val) => val.timestamp, (val) => val.id]))
