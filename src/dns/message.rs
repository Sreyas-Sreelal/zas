use dns::question::Question;
use dns::answer::Answer;

pub struct Message {
    pub id: u16, // 2 bytes
    pub query_response: u16, // 1 bit
    pub operation_code: u16, // 4 bits
    pub authoritative_answer: u16, // 1 bit
    pub truncation_flag: u16, // 1 bit
    pub recursion_desired: u16, // 1 bit
    pub recursion_available: u16, // 1 bit
    pub unused: u16, // 3 bits
    pub error_code: u16, // 4 bits
    pub question_count: u16, // 2 bytes
    pub answer_count: u16, // 2 bytes
    pub ns_count: u16, // 2 bytes
    pub ar_count: u16, // 2 bytes

    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
}

impl Message {
    pub fn unpack(buffer: &[u8]) -> Message {
        let id: u16 = (buffer[0] as u16) << 8 | buffer[1] as u16;
        let body: u16 = (buffer[2] as u16) << 8 | buffer[3] as u16;
        let question_count: u16 = (buffer[4] as u16) << 8 | buffer[5] as u16;
        let answer_count: u16 = (buffer[6] as u16) << 8 | buffer[7] as u16;
        let ns_count: u16 = (buffer[8] as u16) << 8 | buffer[9] as u16;
        let ar_count: u16 = (buffer[10] as u16) << 8 | buffer[11] as u16;

        let mut offset: usize = 12;

        let mut questions = Vec::with_capacity(question_count as usize);

        for _ in 0..question_count {
            match Question::unpack(buffer, offset) {
                (question, updated_offset) => {
                    questions.push(question);
                    offset = updated_offset;
                }
            }
        }

        Message {
            id: id,
            query_response: (body & (1 << 15)) >> 15,
            operation_code: (body & (15 << 11)) >> 11,
            authoritative_answer: (body & (1 >> 10)) >> 10,
            truncation_flag: (body & (1 >> 9)) >> 9,
            recursion_desired: (body & (1 >> 8)) >> 8,
            recursion_available: (body & (1 >> 7)) >> 7,
            unused: (body & (7 << 4)) >> 4,
            error_code: (body & (15 << 0)) >> 0,
            question_count: question_count,
            answer_count: answer_count,
            ns_count: ns_count,
            ar_count: ar_count,
            questions: questions,
            answers: Vec::new(), // TODO parse answers
        }
    }

    pub fn pack(&self, buffer: &mut [u8]) -> usize {
        let mut offset: usize = 0;

        buffer[offset] = (self.id >> 8) as u8;
        buffer[offset + 1] = self.id as u8;
        offset += 2;

        let mut body: u16 = 0;

        body = body | self.query_response << 15;
        body = body | self.operation_code << 11;
        body = body | self.authoritative_answer << 10;
        body = body | self.truncation_flag << 9;
        body = body | self.recursion_desired << 8;
        body = body | self.recursion_available << 7;
        body = body | self.unused << 4;
        body = body | self.error_code;

        buffer[offset] = (body >> 8) as u8;
        buffer[offset + 1] = body as u8;
        offset += 2;

        buffer[offset] = (self.question_count >> 8) as u8;
        buffer[offset + 1] = self.question_count as u8;
        offset += 2;

        buffer[offset] = (self.answer_count >> 8) as u8;
        buffer[offset + 1] = self.answer_count as u8;
        offset += 2;

        buffer[offset] = (self.ns_count >> 8) as u8;
        buffer[offset + 1] = self.ns_count as u8;
        offset += 2;

        buffer[offset] = (self.ar_count >> 8) as u8;
        buffer[offset + 1] = self.ar_count as u8;
        offset += 2;

        for question in self.questions.iter() {
            match question.pack(buffer, offset) {
                updated_offset => {
                    offset = updated_offset;
                }
            }
        }

        for answer in self.answers.iter() {
            for part in answer.name.iter() {
                buffer[offset] = part.len() as u8;
                offset += 1;

                for byte in part.to_owned().into_bytes().iter() {
                    buffer[offset] = *byte;
                    offset += 1;
                }
            }

            buffer[offset] = 0 as u8;
            offset += 1;

            buffer[offset] = (answer.rrtype >> 8) as u8;
            buffer[offset + 1] = answer.rrtype as u8;
            offset += 2;

            buffer[offset] = (answer.class >> 8) as u8;
            buffer[offset + 1] = answer.class as u8;
            offset += 2;

            buffer[offset] = ((answer.ttl & (256 << 24)) >> 24) as u8;
            buffer[offset + 1] = ((answer.ttl & (256 << 16)) >> 16) as u8;
            buffer[offset + 2] = ((answer.ttl & (256 << 8)) >> 8) as u8;
            buffer[offset + 3] = ((answer.ttl & (256 << 0)) >> 0) as u8;
            offset += 4;

            buffer[offset] = (answer.length >> 8) as u8;
            buffer[offset + 1] = answer.length as u8;
            offset += 2;

            for byte in &answer.data {
                buffer[offset] = *byte;
                offset += 1;
            }
        }

        offset
    }
}
