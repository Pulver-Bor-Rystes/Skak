use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct WrappedMsg<M>
where
    M: Serialize,
{
    topic: String,
    payload: Payload<M>,
}

#[derive(Serialize, Debug)]
pub struct Payload<M>
where
    M: Serialize,
{
    result: bool,
    content: M,
}

impl<M> WrappedMsg<M>
where
    M: Serialize + std::marker::Send + std::fmt::Debug,
{
    pub fn payload(topic: impl ToString, content: M) -> WrappedMsg<M> {
        let payload = Payload {
            result: true,
            content,
        };
        WrappedMsg {
            topic: topic.to_string(),
            payload,
        }
    }
}
