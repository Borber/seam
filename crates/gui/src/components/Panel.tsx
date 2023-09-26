import "../css/Panel.css"

import { Accessor, For, Setter } from "solid-js"

import allLives from "../model/Live"

interface LiveProps {
    flag: Setter<boolean>;
    live: Accessor<string>;
    setLive: Setter<string>;
}

const Panel = (props: LiveProps) => {
    return (
        <div
            class="not-draggable panel"
            onMouseEnter={() => props.flag(true)}
            onMouseLeave={() => props.flag(false)}>
            <div class="panel-container">
                <For each={allLives()}>
                    {(item) => (
                        <div
                            class="panel-item"
                            classList={{
                                "panel-item-activate":
                                    props.live() === item.cmd,
                            }}
                            onClick={() => props.setLive(item.cmd)}>
                            {item.name}
                        </div>
                    )}
                </For>
            </div>
        </div>
    )
}

export default Panel
