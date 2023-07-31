import "../css/Bar.css";

import { appWindow } from "@tauri-apps/api/window";
import { Show } from "solid-js";

const Minimize = (
    <svg aria-hidden="false" width="10" height="10" viewBox="0 0 12 12">
        <rect fill="currentColor" width="10" height="1" x="1" y="6" />
    </svg>
);

const Maximize = (
    <svg aria-hidden="false" width="10" height="10" viewBox="0 0 12 12">
        <rect
            width="9"
            height="9"
            x="1.5"
            y="1.5"
            fill="none"
            stroke="currentColor"
        />
    </svg>
);

const Close = (
    <svg aria-hidden="false" width="10" height="10" viewBox="0 0 12 12">
        <polygon
            fill="currentColor"
            fill-rule="evenodd"
            points="11 1.576 6.583 6 11 10.424 10.424 11 6 6.583 1.576 11 1 10.424 5.417 6 1 1.576 1.576 1 6 5.417 10.424 1"
        />
    </svg>
);

export interface BarProps {
    minimize?: boolean;
    maximize?: boolean;
    close?: boolean;
}

const defaultProps: BarProps = {
    minimize: true,
    maximize: true,
    close: true,
};

const Bar = (props: BarProps) => {
    const setting = { ...defaultProps, ...props };
    return (
        <>
            <div data-tauri-drag-region class="top-bar">
                <div class="top-bar-button-container">
                    <Show when={setting.minimize}>
                        <div
                            class="top-bar-button"
                            title="最小化"
                            onClick={() => appWindow.minimize()}>
                            {Minimize}
                        </div>
                    </Show>
                    <Show when={setting.maximize}>
                        <div class="top-bar-button" title="最大化">
                            {Maximize}
                        </div>
                    </Show>
                    <Show when={setting.close}>
                        <div
                            class="top-bar-button top-bar-close"
                            onClick={() => appWindow.close()}
                            title="关闭">
                            {Close}
                        </div>
                    </Show>
                </div>
            </div>
        </>
    );
};

export default Bar;
