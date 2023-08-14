import '../css/Home.css'

import Live from '../components/Live'

const Home = () => {
    const liveDemo = {
        live: 'douyu',
        rid: '123',
        title: '恭喜你发现了我~',
        anchor: '我是谁',
        urls: [],
        img: 'https://i1.hdslb.com/bfs/live/user_cover/9285f31cf9b1f2462a673af1b625b7f201883c32.jpg',
    }
    return (
        <div class="home">
            <Live {...liveDemo} />
            <Live {...liveDemo} />
            <Live {...liveDemo} />
            <Live {...liveDemo} />
            <Live {...liveDemo} />
            <Live {...liveDemo} />
            <Live {...liveDemo} />
            <Live {...liveDemo} />
            <Live {...liveDemo} />
            <Live {...liveDemo} />
            <Live {...liveDemo} />
            <Live {...liveDemo} />
            <Live {...liveDemo} />
            <Live {...liveDemo} />
            <Live {...liveDemo} />
            <Live {...liveDemo} />
            <Live {...liveDemo} />
            <Live {...liveDemo} />
            <Live {...liveDemo} />
            <Live {...liveDemo} />
        </div>
    )
}

export default Home
