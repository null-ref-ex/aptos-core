// Copyright (c) Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

// @generated
/// Generated client implementations.
pub mod safety_rules_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// The SafetyRules gRPC service for remote signing
    #[derive(Debug, Clone)]
    pub struct SafetyRulesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SafetyRulesServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SafetyRulesServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SafetyRulesServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            SafetyRulesServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Get current consensus state
        pub async fn consensus_state(
            &mut self,
            request: impl tonic::IntoRequest<super::ConsensusStateRequest>,
        ) -> std::result::Result<tonic::Response<super::ConsensusStateResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aptos.safety_rules.v1.SafetyRulesService/ConsensusState",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "aptos.safety_rules.v1.SafetyRulesService",
                "ConsensusState",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Initialize with epoch change proof
        pub async fn initialize(
            &mut self,
            request: impl tonic::IntoRequest<super::InitializeRequest>,
        ) -> std::result::Result<tonic::Response<super::InitializeResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aptos.safety_rules.v1.SafetyRulesService/Initialize",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "aptos.safety_rules.v1.SafetyRulesService",
                "Initialize",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Sign a proposal (block data)
        pub async fn sign_proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::SignProposalRequest>,
        ) -> std::result::Result<tonic::Response<super::SignProposalResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aptos.safety_rules.v1.SafetyRulesService/SignProposal",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "aptos.safety_rules.v1.SafetyRulesService",
                "SignProposal",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Sign timeout with QC
        pub async fn sign_timeout_with_qc(
            &mut self,
            request: impl tonic::IntoRequest<super::SignTimeoutWithQcRequest>,
        ) -> std::result::Result<tonic::Response<super::SignTimeoutWithQcResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aptos.safety_rules.v1.SafetyRulesService/SignTimeoutWithQc",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "aptos.safety_rules.v1.SafetyRulesService",
                "SignTimeoutWithQc",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Construct and sign vote (2-chain)
        pub async fn construct_and_sign_vote_two_chain(
            &mut self,
            request: impl tonic::IntoRequest<super::ConstructAndSignVoteTwoChainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConstructAndSignVoteTwoChainResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aptos.safety_rules.v1.SafetyRulesService/ConstructAndSignVoteTwoChain",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "aptos.safety_rules.v1.SafetyRulesService",
                "ConstructAndSignVoteTwoChain",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Construct and sign order vote
        pub async fn construct_and_sign_order_vote(
            &mut self,
            request: impl tonic::IntoRequest<super::ConstructAndSignOrderVoteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConstructAndSignOrderVoteResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aptos.safety_rules.v1.SafetyRulesService/ConstructAndSignOrderVote",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "aptos.safety_rules.v1.SafetyRulesService",
                "ConstructAndSignOrderVote",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Sign commit vote
        pub async fn sign_commit_vote(
            &mut self,
            request: impl tonic::IntoRequest<super::SignCommitVoteRequest>,
        ) -> std::result::Result<tonic::Response<super::SignCommitVoteResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/aptos.safety_rules.v1.SafetyRulesService/SignCommitVote",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "aptos.safety_rules.v1.SafetyRulesService",
                "SignCommitVote",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod safety_rules_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SafetyRulesServiceServer.
    #[async_trait]
    pub trait SafetyRulesService: Send + Sync + 'static {
        /// Get current consensus state
        async fn consensus_state(
            &self,
            request: tonic::Request<super::ConsensusStateRequest>,
        ) -> std::result::Result<tonic::Response<super::ConsensusStateResponse>, tonic::Status>;
        /// Initialize with epoch change proof
        async fn initialize(
            &self,
            request: tonic::Request<super::InitializeRequest>,
        ) -> std::result::Result<tonic::Response<super::InitializeResponse>, tonic::Status>;
        /// Sign a proposal (block data)
        async fn sign_proposal(
            &self,
            request: tonic::Request<super::SignProposalRequest>,
        ) -> std::result::Result<tonic::Response<super::SignProposalResponse>, tonic::Status>;
        /// Sign timeout with QC
        async fn sign_timeout_with_qc(
            &self,
            request: tonic::Request<super::SignTimeoutWithQcRequest>,
        ) -> std::result::Result<tonic::Response<super::SignTimeoutWithQcResponse>, tonic::Status>;
        /// Construct and sign vote (2-chain)
        async fn construct_and_sign_vote_two_chain(
            &self,
            request: tonic::Request<super::ConstructAndSignVoteTwoChainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConstructAndSignVoteTwoChainResponse>,
            tonic::Status,
        >;
        /// Construct and sign order vote
        async fn construct_and_sign_order_vote(
            &self,
            request: tonic::Request<super::ConstructAndSignOrderVoteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConstructAndSignOrderVoteResponse>,
            tonic::Status,
        >;
        /// Sign commit vote
        async fn sign_commit_vote(
            &self,
            request: tonic::Request<super::SignCommitVoteRequest>,
        ) -> std::result::Result<tonic::Response<super::SignCommitVoteResponse>, tonic::Status>;
    }
    /// The SafetyRules gRPC service for remote signing
    #[derive(Debug)]
    pub struct SafetyRulesServiceServer<T: SafetyRulesService> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T: SafetyRulesService> SafetyRulesServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SafetyRulesServiceServer<T>
    where
        T: SafetyRulesService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/aptos.safety_rules.v1.SafetyRulesService/ConsensusState" => {
                    #[allow(non_camel_case_types)]
                    struct ConsensusStateSvc<T: SafetyRulesService>(pub Arc<T>);
                    impl<T: SafetyRulesService>
                        tonic::server::UnaryService<super::ConsensusStateRequest>
                        for ConsensusStateSvc<T>
                    {
                        type Response = super::ConsensusStateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConsensusStateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SafetyRulesService>::consensus_state(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ConsensusStateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aptos.safety_rules.v1.SafetyRulesService/Initialize" => {
                    #[allow(non_camel_case_types)]
                    struct InitializeSvc<T: SafetyRulesService>(pub Arc<T>);
                    impl<T: SafetyRulesService>
                        tonic::server::UnaryService<super::InitializeRequest> for InitializeSvc<T>
                    {
                        type Response = super::InitializeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InitializeRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SafetyRulesService>::initialize(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = InitializeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aptos.safety_rules.v1.SafetyRulesService/SignProposal" => {
                    #[allow(non_camel_case_types)]
                    struct SignProposalSvc<T: SafetyRulesService>(pub Arc<T>);
                    impl<T: SafetyRulesService>
                        tonic::server::UnaryService<super::SignProposalRequest>
                        for SignProposalSvc<T>
                    {
                        type Response = super::SignProposalResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignProposalRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SafetyRulesService>::sign_proposal(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SignProposalSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aptos.safety_rules.v1.SafetyRulesService/SignTimeoutWithQc" => {
                    #[allow(non_camel_case_types)]
                    struct SignTimeoutWithQcSvc<T: SafetyRulesService>(pub Arc<T>);
                    impl<T: SafetyRulesService>
                        tonic::server::UnaryService<super::SignTimeoutWithQcRequest>
                        for SignTimeoutWithQcSvc<T>
                    {
                        type Response = super::SignTimeoutWithQcResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignTimeoutWithQcRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SafetyRulesService>::sign_timeout_with_qc(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SignTimeoutWithQcSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aptos.safety_rules.v1.SafetyRulesService/ConstructAndSignVoteTwoChain" => {
                    #[allow(non_camel_case_types)]
                    struct ConstructAndSignVoteTwoChainSvc<T: SafetyRulesService>(pub Arc<T>);
                    impl<T: SafetyRulesService>
                        tonic::server::UnaryService<super::ConstructAndSignVoteTwoChainRequest>
                        for ConstructAndSignVoteTwoChainSvc<T>
                    {
                        type Response = super::ConstructAndSignVoteTwoChainResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConstructAndSignVoteTwoChainRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SafetyRulesService>::construct_and_sign_vote_two_chain(
                                    &inner, request,
                                )
                                .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ConstructAndSignVoteTwoChainSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aptos.safety_rules.v1.SafetyRulesService/ConstructAndSignOrderVote" => {
                    #[allow(non_camel_case_types)]
                    struct ConstructAndSignOrderVoteSvc<T: SafetyRulesService>(pub Arc<T>);
                    impl<T: SafetyRulesService>
                        tonic::server::UnaryService<super::ConstructAndSignOrderVoteRequest>
                        for ConstructAndSignOrderVoteSvc<T>
                    {
                        type Response = super::ConstructAndSignOrderVoteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConstructAndSignOrderVoteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SafetyRulesService>::construct_and_sign_order_vote(
                                    &inner, request,
                                )
                                .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = ConstructAndSignOrderVoteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/aptos.safety_rules.v1.SafetyRulesService/SignCommitVote" => {
                    #[allow(non_camel_case_types)]
                    struct SignCommitVoteSvc<T: SafetyRulesService>(pub Arc<T>);
                    impl<T: SafetyRulesService>
                        tonic::server::UnaryService<super::SignCommitVoteRequest>
                        for SignCommitVoteSvc<T>
                    {
                        type Response = super::SignCommitVoteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignCommitVoteRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SafetyRulesService>::sign_commit_vote(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SignCommitVoteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", tonic::Code::Unimplemented as i32)
                        .header(http::header::CONTENT_TYPE, tonic::metadata::GRPC_CONTENT_TYPE)
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: SafetyRulesService> Clone for SafetyRulesServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: SafetyRulesService> tonic::server::NamedService for SafetyRulesServiceServer<T> {
        const NAME: &'static str = "aptos.safety_rules.v1.SafetyRulesService";
    }
}
