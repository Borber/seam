import '../css/Setting.css'

import { open } from '@tauri-apps/api/dialog'
import toast from 'solid-toast'

const Setting = () => {
    const save = () => {
        toast.success('保存成功')
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
                        }}
                    >
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

            <div class="setting-title">Headers</div>
            <textarea class="setting-textarea" placeholder="按照官网配置" />
            <button class="setting-save" onClick={() => save()}>
                保存
            </button>
        </div>
    )
}

export default Setting
