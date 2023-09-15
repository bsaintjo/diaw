use std::io::Seek;

use bytes::Buf;

use crate::{
    header::DNSHeader,
    question::DNSQuestion,
    record::{DNSRecord, DNSRecordResult, TYPE_NS},
    TYPE_A,
};

pub struct DNSPacket {
    header: DNSHeader,
    questions: Vec<DNSQuestion>,
    answers: Vec<DNSRecord>,
    authorities: Vec<DNSRecord>,
    additionals: Vec<DNSRecord>,
}

fn parse_records<B>(data: &mut B, num_records: u16) -> Vec<DNSRecord>
where
    B: Buf + Seek,
{
    (0..num_records)
        .map(|_| DNSRecord::parse_record(data))
        .collect()
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
        let answers = parse_records(data, header.num_answers);
        let authorities = parse_records(data, header.num_authorities);
        let additionals = parse_records(data, header.num_additionals);
        DNSPacket {
            header,
            questions,
            answers,
            authorities,
            additionals,
        }
    }

    /// Returns the first A record in the answers section
    pub fn get_answer(&self) -> Option<&DNSRecordResult> {
        self.answers
            .iter()
            .find(|r| r.qtype() == TYPE_A)
            .map(|r| r.res())
    }

    /// Returns the first A record in the additionals section
    pub fn get_nameserver_ip(&self) -> Option<&DNSRecordResult> {
        self.additionals
            .iter()
            .find(|r| r.qtype() == TYPE_A)
            .map(|r| r.res())
    }

    pub fn get_nameserver(&self) -> Option<&DNSRecordResult> {
        self.authorities
            .iter()
            .find(|r| r.qtype() == TYPE_NS)
            .map(|r| r.res())
    }

    pub fn answers(&self) -> &[DNSRecord] {
        self.answers.as_ref()
    }

    pub fn authorities(&self) -> &[DNSRecord] {
        self.authorities.as_ref()
    }

    pub fn additionals(&self) -> &[DNSRecord] {
        self.additionals.as_ref()
    }
}
