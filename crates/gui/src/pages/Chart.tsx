import '../css/Chart.css'

import { createSignal, For } from 'solid-js'

import allLives from '../model/Live'

const Chart = () => {
    const [selected, setSelect] = createSignal('all')
    return (
        <div class="chart">
            <div class="chart-kind-container">
                <div
                    class="chart-kind-item"
                    classList={{
                        'chart-kind-item-activate': selected() === 'all',
                    }}
                    onClick={() => setSelect('all')}
                >
                    ALL
                </div>
                <For each={allLives()}>
                    {(item) => (
                        <div
                            class="chart-kind-item"
                            classList={{
                                'chart-kind-item-activate':
                                    selected() === item.cmd,
                            }}
                            onClick={() => setSelect(item.cmd)}
                        >
                            {item.name}
                        </div>
                    )}
                </For>
            </div>
        </div>
    )
}

export default Chart
