import '../css/Home.css'

import Live from '../components/Live'

const Home = () => {
    const liveDemo = {
        live: 'douyu',
        rid: '123',
        title: '恭喜你发现了我~',
        anchor: '我是谁',
        urls: [],
        img: undefined,
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
