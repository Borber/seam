import '../css/Good.css'

import GoodItem from '../components/GoodItem'

const Good = () => {
    const goodDemo = {
        live: 'douyu',
        rid: '123',
        title: '恭喜你发现了我~',
        anchor: '我是谁',
        urls: [],
        img: undefined,
    }
    return (
        <div class="good">
            <GoodItem {...goodDemo} />
            <GoodItem {...goodDemo} />
            <GoodItem {...goodDemo} />
            <GoodItem {...goodDemo} />
            <GoodItem {...goodDemo} />
        </div>
    )
}

export default Good
