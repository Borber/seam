import { AddIcon, SyncIcon } from '../icon/icon'

const TopBar = () => {
    return (
        <div data-tauri-drag-region class="top-bar">
            <button class="top-bar-btn">
                <SyncIcon size={18} />
            </button>
            <input placeholder="房间号" class="top-bar-input" />
            <button class="top-bar-btn">
                <AddIcon size={16} />
            </button>
        </div>
    )
}

export default TopBar
