import '../css/Good.css'

import GoodItem from '../components/GoodItem'

const Good = () => {
    const goodDemo = {
        live: 'douyu',
        rid: '123',
        title: '恭喜你发现了我~',
        anchor: '我是谁',
        urls: [],
        img: 'https://i1.hdslb.com/bfs/live/user_cover/9285f31cf9b1f2462a673af1b625b7f201883c32.jpg',
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
