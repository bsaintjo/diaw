use std::io::Seek;

use bytes::Buf;

use crate::{header::DNSHeader, question::DNSQuestion, record::DNSRecord};

pub struct DNSPacket {
    header: DNSHeader,
    questions: Vec<DNSQuestion>,
    answers: Vec<DNSRecord>,
    authorities: Vec<DNSRecord>,
    additionals: Vec<DNSRecord>,
}

impl DNSPacket {
    pub fn parse_dns_packet<B>(data: &mut B) -> Self
    where
        B: Buf + Seek,
    {
        let header = DNSHeader::parse_header(data);
        let questions = (0..header.num_questions)
            .map(|_| DNSQuestion::parse_question(data))
            .collect();
        let answers = (0..header.num_answers)
            .map(|_| DNSRecord::parse_record(data))
            .collect();
        let authorities = (0..header.num_authorities)
            .map(|_| DNSRecord::parse_record(data))
            .collect();
        let additionals = (0..header.num_additionals)
            .map(|_| DNSRecord::parse_record(data))
            .collect();
        DNSPacket {
            header,
            questions,
            answers,
            authorities,
            additionals,
        }
    }

    pub fn answers(&self) -> &[DNSRecord] {
        self.answers.as_ref()
    }
}
