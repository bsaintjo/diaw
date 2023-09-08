use bytes::Buf;

#[derive(Debug)]
pub struct DNSHeader {
    pub id: u16,
    pub flags: u16,
    pub num_questions: u16,
    pub num_answers: u16,
    pub num_authorities: u16,
    pub num_additionals: u16,
}

impl DNSHeader {
    pub fn new(id: u16, flags: u16) -> DNSHeader {
        DNSHeader {
            id,
            flags,
            num_questions: 0,
            num_answers: 0,
            num_authorities: 0,
            num_additionals: 0,
        }
    }

    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(12);
        // bytes.extend_from_slice(&0x8290_u16.to_be_bytes());
        bytes.extend_from_slice(&self.id.to_be_bytes());
        bytes.extend_from_slice(&self.flags.to_be_bytes());
        bytes.extend_from_slice(&self.num_questions.to_be_bytes());
        bytes.extend_from_slice(&self.num_answers.to_be_bytes());
        bytes.extend_from_slice(&self.num_authorities.to_be_bytes());
        bytes.extend_from_slice(&self.num_additionals.to_be_bytes());
        bytes
    }

    pub fn parse_header<B: Buf>(bytes: &mut B) -> Self {
        let id = bytes.get_u16();
        let flags = bytes.get_u16();
        let num_questions = bytes.get_u16();
        let num_answers = bytes.get_u16();
        let num_authorities = bytes.get_u16();
        let num_additionals = bytes.get_u16();
        Self {
            id,
            flags,
            num_questions,
            num_answers,
            num_authorities,
            num_additionals,
        }
    }
}
