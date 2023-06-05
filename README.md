In Memory is a small Radius-line library for storing information in the application itself, without using third-party services. The data is stored in key-value format, as in Redis or HashMap.

Uses Tokio and Bincode libraries


    use inmemory::Manager;

    #[tokio::main]
    async fn main() {

        let memory = Manager::new()
            // One MB memory limit
            .limit(1024 * 1024) // it is not necessary to specify
            // The GC will check the memory once at a given time in seconds.
            .interval(60 * 60) // it is not necessary to specify
            // Starting the GC and returning the Memory object
            .run().await;

        // Add memory to the service. 
        // The following are specified: key, data and lifetime.
        let _result = memory.add(0, String::from("Some data"), 60* 60).await;
    
        // Getting data from the service, the key is specified.
        let _result = memory.get(0).await;
        
        // Manual activation of GC to check "dead" values at any convenient time.
        // It works even if GC is disabled.
        let _result = memory.clear().await;

    }


The garbage collector is made as simple as possible, without any algorithms - the check is carried out once at the specified time in **interval_sec**. If you specify 0 or not at all, the collector will not work and the memory will have to be cleaned exclusively manually.

