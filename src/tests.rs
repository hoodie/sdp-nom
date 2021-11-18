use super::*;

#[test]
fn anatomy() {
    //! every exaple from<https://webrtchacks.com/sdp-anatomy/>

    let anatomy_examples = [
            // Global Lines
            "o=- 4611731400430051336 2 IN IP4 127.0.0.1",
            "s=-",
            "t=0 0",
            "a=group:BUNDLE 0 1",
            "a=msid-semantic: WMS lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS",
            "m=audio 58779 UDP/TLS/RTP/SAVPF 111 103 104 9 0 8 106 105 13 126",
            "c=IN IP4 217.130.243.155",
            "a=rtcp:51472 IN IP4 217.130.243.155",

            // Audio Lines
            "a=candidate:1467250027 1 udp 2122260223 192.168.0.196 46243 typ host generation 0",
            "a=candidate:1467250027 2 udp 2122260222 192.168.0.196 56280 typ host generation 0",
            "a=candidate:435653019 1 tcp 1845501695 192.168.0.196 0 typ host tcptype active generation 0",
            "a=candidate:435653019 2 tcp 1845501695 192.168.0.196 0 typ host tcptype active generation 0",
            "a=candidate:1853887674 1 udp 1518280447 47.61.61.61 36768 typ srflx raddr 192.168.0.196 rport 36768 generation 0",
            "a=candidate:1853887674 2 udp 1518280447 47.61.61.61 36768 typ srflx raddr 192.168.0.196 rport 36768 generation 0",
            "a=candidate:750991856 2 udp 25108222 237.30.30.30 51472 typ relay raddr 47.61.61.61 rport 54763 generation 0",
            "a=candidate:750991856 1 udp 25108223 237.30.30.30 58779 typ relay raddr 47.61.61.61 rport 54761 generation 0",

            // ICE Parameters
            "a=ice-ufrag:Oyef7uvBlwafI3hT",
            "a=ice-pwd:T0teqPLNQQOf+5W+ls+P2p16",

            // DTLS Parameters
            "a=fingerprint:sha-256 49:66:12:17:0D:1C:91:AE:57:4C:C6:36:DD:D5:97:D2:7D:62:C9:9A:7F:B9:A3:F4:70:03:E7:43:91:73:23:5E",
            "a=setup:actpass",
            "a=mid:0",
            "a=extmap:1 urn:ietf:params:rtp-hdrext:ssrc-audio-level",
            "a=extmap:3 http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time",
            "a=sendrecv",
            "a=rtcp-mux",

            // Codec Parameters
            "a=rtpmap:111 opus/48000/2",
            "a=fmtp:111 minptime=10; useinbandfec=1",
            "a=rtpmap:103 ISAC/16000",
            "a=rtpmap:104 ISAC/32000",
            "a=rtpmap:9 G722/8000",
            "a=rtpmap:0 PCMU/8000",
            "a=rtpmap:8 PCMA/8000",
            "a=rtpmap:106 CN/32000",
            "a=rtpmap:105 CN/16000",
            "a=rtpmap:13 CN/8000",
            "a=rtpmap:126 telephone-event/8000",
            "a=maxptime:60",

            //SSRC Parameters
            "a=ssrc:3570614608 cname:4TOk42mSjXCkVIa6",
            "a=ssrc:3570614608 msid:lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS 35429d94-5637-4686-9ecd-7d0622261ce8",
            "a=ssrc:3570614608 mslabel:lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS",
            "a=ssrc:3570614608 label:35429d94-5637-4686-9ecd-7d0622261ce8",

            // Video Lines
            "m=video 60372 UDP/TLS/RTP/SAVPF 100 101 116 117 96",
            "c=IN IP4 217.130.243.155",
            "a=rtcp:64891 IN IP4 217.130.243.155",

            // ICE Candidates
            "a=candidate:1467250027 1 udp 2122260223 192.168.0.196 56143 typ host generation 0",
            "a=candidate:1467250027 2 udp 2122260222 192.168.0.196 58874 typ host generation 0",
            "a=candidate:435653019 1 tcp 1518280447 192.168.0.196 0 typ host tcptype active generation 0",
            "a=candidate:435653019 2 tcp 1518280446 192.168.0.196 0 typ host tcptype active generation 0",
            "a=candidate:1853887674 1 udp 1518280447 47.61.61.61 36768 typ srflx raddr 192.168.0.196 rport 36768 generation 0",
            "a=candidate:1853887674 1 udp 1518280447 47.61.61.61 36768 typ srflx raddr 192.168.0.196 rport 36768 generation 0",
            "a=candidate:750991856 1 udp 25108223 237.30.30.30 60372 typ relay raddr 47.61.61.61 rport 54765 generation 0",
            "a=candidate:750991856 2 udp 25108222 237.30.30.30 64891 typ relay raddr 47.61.61.61 rport 54767 generation 0",

            // ICE Parameters
            "a=ice-ufrag:Oyef7uvBlwafI3hT",
            "a=ice-pwd:T0teqPLNQQOf+5W+ls+P2p16",

            // DTLS Parameters
            "a=fingerprint:sha-256 49:66:12:17:0D:1C:91:AE:57:4C:C6:36:DD:D5:97:D2:7D:62:C9:9A:7F:B9:A3:F4:70:03:E7:43:91:73:23:5E",
            "a=setup:actpass",
            "a=mid:1",
            "a=extmap:2 urn:ietf:params:rtp-hdrext:toffset",
            "a=extmap:3 http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time",
            "a=extmap:4 urn:3gpp:video-orientation",
            "a=sendrecv",
            "a=rtcp-mux",

            // Codec Parameters
            "a=rtpmap:100 VP8/90000",
            "a=rtcp-fb:100 ccm fir",
            "a=rtcp-fb:100 nack",
            "a=rtcp-fb:100 nack pli",
            "a=rtcp-fb:100 goog-remb",
            "a=rtpmap:101 VP9/90000",
            "a=rtcp-fb:101 ccm fir",
            "a=rtcp-fb:101 nack",
            "a=rtcp-fb:101 nack pli",
            "a=rtcp-fb:101 goog-remb",
            "a=rtpmap:116 red/90000",
            "a=rtpmap:117 ulpfec/90000",
            "a=rtpmap:96 rtx/90000",
            "a=fmtp:96 apt=100",

            // SSRC Parameters
            "a=ssrc-group:FID 2231627014 632943048",
            "a=ssrc:2231627014 cname:4TOk42mSjXCkVIa6",
            "a=ssrc:2231627014 msid:lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS daed9400-d0dd-4db3-b949-422499e96e2d",
            "a=ssrc:2231627014 mslabel:lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS",
            "a=ssrc:2231627014 label:daed9400-d0dd-4db3-b949-422499e96e2d",
            "a=ssrc:632943048 cname:4TOk42mSjXCkVIa6",
            "a=ssrc:632943048 msid:lgsCFqt9kN2fVKw5wg3NKqGdATQoltEwOdMS daed9400-d0dd-4db3-b949-422499e96e2d",
        ];
    for (i, line) in anatomy_examples.iter().enumerate() {
        print!("{}.", i);
        assert_line!(line);
    }
}

#[test]
fn seldom_lines() {
    let seldom_lines = [
        "i=foobar",
        "e=email@example.com",
        "p=0118 999 881 999 119 7253",
    ];
    for (i, line) in seldom_lines.iter().enumerate() {
        print!("{}.", i);
        assert_line!(line);
    }
}
