import "../css/Setting.css"

import { open } from "@tauri-apps/api/dialog"
import toast from "solid-toast"

// TODO headers 设置分离, 取消 textarea, 列表,
// 顶部添加, 选择平台, 输入字段, 值, 点击添加
// 列表最右边添加删除按钮, 点击删除
// 列表的值可以修改, 点击修改

const Setting = () => {
    const save = () => {
        toast.success("保存成功")
    }
    return (
        <div class="setting">
            <div class="setting-title">播放</div>
            <div class="setting-item">
                <div class="setting-item-title">播放器</div>
                <div>
                    <input class="setting-input" placeholder="命令/地址" />
                    <button
                        class="setting-btn"
                        onClick={async () => {
                            const file = await open()
                            console.log(file)
                        }}>
                        选择
                    </button>
                </div>
            </div>
            <div class="setting-item">
                <div class="setting-item-title">参数</div>
                <input
                    class="setting-input setting-arg"
                    placeholder="逗号分隔"
                />
            </div>

            {/* TODO 将目前已知需要额外配置的 cookie 写入此处, 让用户知道哪些需要额外配置 */}
            <div class="setting-title">Headers</div>
            <textarea class="setting-textarea" placeholder="按照官网配置" />
            <button class="setting-save" onClick={() => save()}>
                保存
            </button>
        </div>
    )
}

export default Setting
