// Assign roles to admins
role(actor: String, "admin") if
    actor = "Gray" or
    actor = "Alice";

// Grant admin privilege of retrieving company's job. 
allow(actor: String, "GET", _job: Job) if
    role(actor, "admin");
