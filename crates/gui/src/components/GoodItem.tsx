import '../css/GoodItem.css'

import { AddIcon, CopyIcon, PlayIcon } from '../icon/icon'

interface Url {
    format: string
    url: string
}

interface GoodItemProps {
    live: string
    rid: string
    title: string
    anchor: string
    urls: Url[]
    img: string
}

const GoodItem = (props: GoodItemProps) => {
    return (
        <div class="good-item">
            <img class="good-img" src={props.img} />
            <div class="good-panel">
                <div class="good-title">{props.title}</div>
                <div class="good-info">快来看</div>
                <div class="good-control">
                    <button class="good-control-btn">
                        <AddIcon size={15} />
                    </button>
                    <button class="good-control-btn">
                        <CopyIcon size={15} />
                    </button>
                    <button class="good-control-btn">
                        <PlayIcon size={15} />
                    </button>
                </div>
            </div>
        </div>
    )
}

export default GoodItem
