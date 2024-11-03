pub mod v0 {

    #[derive(Debug, Clone)]
    pub struct Topic {
        pub name: String,
    }

    #[derive(Debug, Clone)]
    pub struct MetadataRequest {
        pub topics: Vec<Topic>,
    }

    #[derive(Debug, Clone)]
    pub struct Broker {
        pub node_id: i32,
        pub host: String,
        pub port: i32,
    }

    #[derive(Debug, Clone)]
    pub struct PartitionMetadata {
        pub error_code: i16,
        pub partition_index: i32,
        pub leader_id: i32,
        pub replica_nodes: i32,
        pub isr_nodes: i32,
    }

    #[derive(Debug, Clone)]
    pub struct TopicMetadata {
        pub error_code: i32,
        pub name: String,
        pub partitions: Vec<PartitionMetadata>,
    }

    #[derive(Debug, Clone)]
    pub struct MetadataResponse {
        pub brokers: Vec<Broker>,
        pub topics: Vec<TopicMetadata>,
    }

    pub fn to_bytes(request: MetadataRequest, correlation_id: i32, buf: &mut Vec<u8>) {
        // The API key for metadata is 3
        buf.extend_from_slice(&i16::to_be_bytes(3));
        // Version is 0
        buf.extend_from_slice(&[0, 0]);
        // Write correlation_id
        buf.extend_from_slice(&i32::to_be_bytes(correlation_id));
        // Write the client id
        buf.extend_from_slice(&i16::to_be_bytes(-1));
        // Write the request
        buf.extend_from_slice(&i32::to_be_bytes(request.topics.len().try_into().unwrap()));
        for topic in request.topics.iter() {
            buf.extend_from_slice(&i16::to_be_bytes(topic.name.len().try_into().unwrap()));
            buf.extend_from_slice(topic.name.as_bytes());
        }
    }
}
