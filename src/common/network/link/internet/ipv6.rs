pub struct IPv6Header {
    /*pub traffic_class: u8,
    pub flow_label: Vec<u8>,
    pub payload_length: u16,
    pub next_header: u8,
    pub hop_limit: u8,
    pub src_addr: u64,
    pub dst_addr: u64*/
}

impl IPv6Header {
    pub(crate) fn new(bytes: &[u8]) -> IPv6Header {
        IPv6Header {}
    }
}