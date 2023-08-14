import '../css/Chart.css'

import { createMemo, createSignal, For } from 'solid-js'
import toast from 'solid-toast'

import allLives from '../model/Live'

interface Record {
    index: number
    live: string
    rid: string
    anchor: string
}

const Chart = () => {
    const [selected, setSelect] = createSignal('all')
    const [records, setRecords] = createSignal<Record[]>([
        {
            index: 1,
            live: 'douyu',
            rid: '123',
            anchor: 'AAAA',
        },
        {
            index: 2,
            live: 'bili',
            rid: '123',
            anchor: 'AAAA',
        },
        {
            index: 3,
            live: 'bili',
            rid: '123',
            anchor: 'AAAA',
        },
        {
            index: 4,
            live: 'yqs',
            rid: '123',
            anchor: 'AAAA',
        },
        {
            index: 5,
            live: 'cc',
            rid: '123',
            anchor: 'AAAA',
        },
        {
            index: 6,
            live: 'kk',
            rid: '123',
            anchor: 'AAAA',
        },
        {
            index: 7,
            live: 'douyu',
            rid: '123',
            anchor: 'AAAA',
        },
        {
            index: 8,
            live: 'douyu',
            rid: '123',
            anchor: 'AAAA',
        },
        {
            index: 9,
            live: 'douyu',
            rid: '123',
            anchor: 'AAAA',
        },
        {
            index: 10,
            live: 'douyu',
            rid: '123',
            anchor: 'AAAA',
        },
        {
            index: 11,
            live: 'douyu',
            rid: '123',
            anchor: 'AAAA',
        },
        {
            index: 12,
            live: 'douyu',
            rid: '123',
            anchor: 'AAAA',
        },
        {
            index: 13,
            live: 'douyu',
            rid: '123',
            anchor: 'AAAA',
        },
        {
            index: 14,
            live: 'douyu',
            rid: '123',
            anchor: 'AAAA',
        },
    ])

    const filterRecords = createMemo(() => {
        if (selected() === 'all') {
            return records()
        } else {
            return records().filter((item) => item.live === selected())
        }
    })

    const deleteRecord = (index: number) => {
        setRecords(records().filter((item) => item.index !== index))
        toast.success('删除成功')
    }

    const liveName = (live: string) => {
        const lives = allLives()
        for (let i = 0; i < lives.length; i++) {
            if (lives[i].cmd === live) {
                return lives[i].name
            }
        }
        return '未知'
    }
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
            <table class="chart-table">
                <thead>
                    <tr>
                        <th class="chart-table-title-1">平台</th>
                        <th class="chart-table-title-2">房间号</th>
                        <th class="chart-table-title-3">主播</th>
                        <th class="chart-table-title-4">操作</th>
                    </tr>
                </thead>
                <tbody>
                    <For each={filterRecords()}>
                        {(item) => (
                            <tr>
                                <td>{liveName(item.live)}</td>
                                <td>{item.rid}</td>
                                <td>{item.anchor}</td>
                                <td>
                                    <button
                                        onClick={() => {
                                            deleteRecord(item.index)
                                        }}
                                    >
                                        删除
                                    </button>
                                </td>
                            </tr>
                        )}
                    </For>
                </tbody>
            </table>
            <div class="chart-separator" />
        </div>
    )
}

export default Chart
