// Assign roles to admins
role(actor: String, "admin") if
    actor = "Gray" or
    actor = "Alice";

// Assign roles to receptionist
role(actor: String, "receptionist") if
    actor = "Drake";

// Grant admins privilege to retrieve firm's jobs
allow(actor: String, "GET", _job: Job) if
    role(actor, "admin");

// Grant receptionist privilege to view firm's jobs
allow(actor: String, "view", _job: Job) if
    role(actor, "receptionist");
