import "../css/Chart.css"

import { invoke } from "@tauri-apps/api"
import { createMemo, createSignal, For, onMount } from "solid-js"
import toast from "solid-toast"

import allLives from "../model/Live"
import { Resp } from "../model/Resp"

interface Record {
    index: number
    live: string
    rid: string
    anchor: string
}

interface Subscribe {
    live: string
    rid: string
}

const Chart = () => {
    const [selected, setSelect] = createSignal("all")
    const [records, setRecords] = createSignal<Record[]>([])

    // 开启页面获取 records 数据

    onMount(async () => {
        const subscribes = await invoke<Resp<Subscribe[]>>("subscribe_all")
        console.log(subscribes)
        const map = subscribes.data.map((item, index) => {
            return {
                index: index,
                live: item.live,
                rid: item.rid,
                anchor: "未知",
            }
            setRecords(map)
        })
        setRecords(map)
    })

    const filterRecords = createMemo(() => {
        if (selected() === "all") {
            return records()
        } else {
            return records().filter((item) => item.live === selected())
        }
    })

    const deleteRecord = async (index: number) => {
        const item = records().find((item) => item.index === index)
        setRecords(records().filter((item) => item.index !== index))
        await invoke<Resp<Subscribe[]>>("subscribe_remove", {
            live: item?.live,
            rid: item?.rid,
        }).then((resp) => {
            if (resp.code === 0) {
                toast.success("删除成功")
            } else {
                toast.error(resp.msg)
            }
        })
    }

    const liveName = (live: string) => {
        const lives = allLives()
        for (let i = 0; i < lives.length; i++) {
            if (lives[i].cmd === live) {
                return lives[i].name
            }
        }
        return "未知"
    }

    return (
        <div class="chart">
            <div class="chart-kind-container">
                <div
                    class="chart-kind-item"
                    classList={{
                        "chart-kind-item-activate": selected() === "all",
                    }}
                    onClick={() => setSelect("all")}>
                    ALL
                </div>
                <For each={allLives()}>
                    {(item) => (
                        <div
                            class="chart-kind-item"
                            classList={{
                                "chart-kind-item-activate":
                                    selected() === item.cmd,
                            }}
                            onClick={() => setSelect(item.cmd)}>
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
                                        }}>
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
