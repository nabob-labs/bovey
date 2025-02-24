use {crossbeam_channel::Receiver, bovey_perf::packet::PacketBatch, std::sync::Arc};

pub type BankingPacketBatch = Arc<Vec<PacketBatch>>;
pub type BankingPacketReceiver = Receiver<BankingPacketBatch>;
