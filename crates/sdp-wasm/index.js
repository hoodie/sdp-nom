const content = `v=0
o=- 3383575101619166804 4 IN IP4 127.0.0.1
s=-
t=0 0
a=group:BUNDLE 1
a=extmap-allow-mixed
a=msid-semantic: WMS wYMHgchcIvfAdYxfDSJJhiVLyaTKph1xiqr8
m=video 54138 UDP/TLS/RTP/SAVPF 96 97 98 99 100 101 122 102 121 127 120 125 107 108 109 124 119 123 118 114 115 116 35
c=IN IP4 10.42.23.48
a=rtcp:9 IN IP4 0.0.0.0
a=candidate:2791055836 1 udp 2122262783 2001:abc:b0b:cafe:babe:dead:beef:ab0b 58605 typ host generation 0 network-id 2
a=candidate:53709098 1 udp 2122197247 2001:abc:b0b:cafe:babe:dead:beef:ab0b 63575 typ host generation 0 network-id 4
a=candidate:2400687251 1 udp 2122129151 10.42.23.48 54138 typ host generation 0 network-id 1
a=candidate:584466674 1 udp 2122063615 10.42.23.63 55016 typ host generation 0 network-id 3
a=candidate:1303410138 1 tcp 1518217471 2001:abc:b0b:cafe:babe:dead:beef:ab0b 9 typ host tcptype active generation 0 network-id 4
a=candidate:3247728739 1 tcp 1518149375 10.42.23.48 9 typ host tcptype active generation 0 network-id 1
a=candidate:1817558018 1 tcp 1518083839 10.42.23.63 9 typ host tcptype active generation 0 network-id 3
a=ice-ufrag:Owwj
a=ice-pwd:UqC5WDoQi1pav4+INNtEdZHk
a=ice-options:trickle
a=fingerprint:sha-256 97:DD:B6:B4:69:1C:A9:A7:56:90:4D:B9:8E:22:0A:B3:51:1A:72:CA:7F:A7:02:58:2B:2A:8E:C4:F7:90:09:F8
a=setup:actpass
a=mid:1
a=extmap:1 urn:ietf:params:rtp-hdrext:toffset
a=extmap:2 http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time
a=extmap:3 urn:3gpp:video-orientation
a=extmap:4 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01
a=extmap:5 http://www.webrtc.org/experiments/rtp-hdrext/playout-delay
a=extmap:6 http://www.webrtc.org/experiments/rtp-hdrext/video-content-type
a=extmap:7 http://www.webrtc.org/experiments/rtp-hdrext/video-timing
a=extmap:8 http://www.webrtc.org/experiments/rtp-hdrext/color-space
a=extmap:9 urn:ietf:params:rtp-hdrext:sdes:mid
a=extmap:10 urn:ietf:params:rtp-hdrext:sdes:rtp-stream-id
a=extmap:11 urn:ietf:params:rtp-hdrext:sdes:repaired-rtp-stream-id
a=sendrecv
a=msid:wYMHgchcIvfAdYxfDSJJhiVLyaTKph1xiqr8 86d315d4-436a-4d83-bda9-01fdf303dccc
a=rtcp-mux
a=rtcp-rsize
a=rtpmap:96 VP8/90000
a=rtcp-fb:96 goog-remb
a=rtcp-fb:96 transport-cc
a=rtcp-fb:96 ccm fir
a=rtcp-fb:96 nack
a=rtcp-fb:96 nack pli
a=rtpmap:97 rtx/90000
a=fmtp:97 apt=96
a=rtpmap:98 VP9/90000
a=rtcp-fb:98 goog-remb
a=rtcp-fb:98 transport-cc
a=rtcp-fb:98 ccm fir
a=rtcp-fb:98 nack
a=rtcp-fb:98 nack pli
a=fmtp:98 profile-id=0
a=rtpmap:99 rtx/90000
a=fmtp:99 apt=98
a=rtpmap:100 VP9/90000
a=rtcp-fb:100 goog-remb
a=rtcp-fb:100 transport-cc
a=rtcp-fb:100 ccm fir
a=rtcp-fb:100 nack
a=rtcp-fb:100 nack pli
a=fmtp:100 profile-id=2
a=rtpmap:101 rtx/90000
a=fmtp:101 apt=100
a=rtpmap:122 VP9/90000
a=rtcp-fb:122 goog-remb
a=rtcp-fb:122 transport-cc
a=rtcp-fb:122 ccm fir
a=rtcp-fb:122 nack
a=rtcp-fb:122 nack pli
a=fmtp:122 profile-id=1
a=rtpmap:102 H264/90000
a=rtcp-fb:102 goog-remb
a=rtcp-fb:102 transport-cc
a=rtcp-fb:102 ccm fir
a=rtcp-fb:102 nack
a=rtcp-fb:102 nack pli
a=fmtp:102 level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=42001f
a=rtpmap:121 rtx/90000
a=fmtp:121 apt=102
a=rtpmap:127 H264/90000
a=rtcp-fb:127 goog-remb
a=rtcp-fb:127 transport-cc
a=rtcp-fb:127 ccm fir
a=rtcp-fb:127 nack
a=rtcp-fb:127 nack pli
a=fmtp:127 level-asymmetry-allowed=1;packetization-mode=0;profile-level-id=42001f
a=rtpmap:120 rtx/90000
a=fmtp:120 apt=127
a=rtpmap:125 H264/90000
a=rtcp-fb:125 goog-remb
a=rtcp-fb:125 transport-cc
a=rtcp-fb:125 ccm fir
a=rtcp-fb:125 nack
a=rtcp-fb:125 nack pli
a=fmtp:125 level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=42e01f
a=rtpmap:107 rtx/90000
a=fmtp:107 apt=125
a=rtpmap:108 H264/90000
a=rtcp-fb:108 goog-remb
a=rtcp-fb:108 transport-cc
a=rtcp-fb:108 ccm fir
a=rtcp-fb:108 nack
a=rtcp-fb:108 nack pli
a=fmtp:108 level-asymmetry-allowed=1;packetization-mode=0;profile-level-id=42e01f
a=rtpmap:109 rtx/90000
a=fmtp:109 apt=108
a=rtpmap:124 H264/90000
a=rtcp-fb:124 goog-remb
a=rtcp-fb:124 transport-cc
a=rtcp-fb:124 ccm fir
a=rtcp-fb:124 nack
a=rtcp-fb:124 nack pli
a=fmtp:124 level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=4d0032
a=rtpmap:119 rtx/90000
a=fmtp:119 apt=124
a=rtpmap:123 H264/90000
a=rtcp-fb:123 goog-remb
a=rtcp-fb:123 transport-cc
a=rtcp-fb:123 ccm fir
a=rtcp-fb:123 nack
a=rtcp-fb:123 nack pli
a=fmtp:123 level-asymmetry-allowed=1;packetization-mode=1;profile-level-id=640032
a=rtpmap:118 rtx/90000
a=fmtp:118 apt=123
a=rtpmap:114 red/90000
a=rtpmap:115 rtx/90000
a=fmtp:115 apt=114
a=rtpmap:116 ulpfec/90000
a=rtpmap:35 flexfec-03/90000
a=rtcp-fb:35 goog-remb
a=rtcp-fb:35 transport-cc
a=fmtp:35 repair-window=10000000
a=ssrc-group:FID 306837937 4049932520
a=ssrc:306837937 cname:mNlgMpULjnGBD+Bt
a=ssrc:306837937 msid:wYMHgchcIvfAdYxfDSJJhiVLyaTKph1xiqr8 86d315d4-436a-4d83-bda9-01fdf303dccc
a=ssrc:306837937 mslabel:wYMHgchcIvfAdYxfDSJJhiVLyaTKph1xiqr8
a=ssrc:306837937 label:86d315d4-436a-4d83-bda9-01fdf303dccc
a=ssrc:4049932520 cname:mNlgMpULjnGBD+Bt
a=ssrc:4049932520 msid:wYMHgchcIvfAdYxfDSJJhiVLyaTKph1xiqr8 86d315d4-436a-4d83-bda9-01fdf303dccc
a=ssrc:4049932520 mslabel:wYMHgchcIvfAdYxfDSJJhiVLyaTKph1xiqr8
a=ssrc:4049932520 label:86d315d4-436a-4d83-bda9-01fdf303dccc`

async function main() {
    const sdpWasm = await import("./pkg");
    window.sdpWasm = sdpWasm;
    window.demoSdp = content;

    window.sdp = sdpWasm.parse_sdp(content);

    console.debug({sdpWasm})
}

main();
