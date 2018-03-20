(function() {var implementors = {};
implementors["strymon_communication"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"strymon_communication/transport/struct.Sender.html\" title=\"struct strymon_communication::transport::Sender\">Sender</a>","impl&lt;N:&nbsp;<a class=\"trait\" href=\"strymon_communication/rpc/trait.Name.html\" title=\"trait strymon_communication::rpc::Name\">Name</a>, R:&nbsp;<a class=\"trait\" href=\"strymon_communication/rpc/trait.Request.html\" title=\"trait strymon_communication::rpc::Request\">Request</a>&lt;N&gt;&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"strymon_communication/rpc/struct.Response.html\" title=\"struct strymon_communication::rpc::Response\">Response</a>&lt;N, R&gt;","impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"strymon_communication/fetch/struct.Handle.html\" title=\"struct strymon_communication::fetch::Handle\">Handle</a>",];
implementors["strymon_coordinator"] = ["impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"strymon_coordinator/handler/struct.CoordinatorRef.html\" title=\"struct strymon_coordinator::handler::CoordinatorRef\">CoordinatorRef</a>",];
implementors["strymon_job"] = ["impl&lt;T:&nbsp;Timestamp, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"strymon_job/operators/subscribe/struct.Subscription.html\" title=\"struct strymon_job::operators::subscribe::Subscription\">Subscription</a>&lt;T, D&gt;","impl&lt;N:&nbsp;<a class=\"trait\" href=\"strymon_communication/rpc/trait.Name.html\" title=\"trait strymon_communication::rpc::Name\">Name</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"strymon_job/operators/service/struct.Service.html\" title=\"struct strymon_job::operators::service::Service\">Service</a>&lt;N&gt;","impl&lt;N:&nbsp;<a class=\"trait\" href=\"strymon_communication/rpc/trait.Name.html\" title=\"trait strymon_communication::rpc::Name\">Name</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"strymon_job/operators/service/struct.Client.html\" title=\"struct strymon_job::operators::service::Client\">Client</a>&lt;N&gt;",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()