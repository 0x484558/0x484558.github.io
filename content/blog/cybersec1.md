+++
title = "Cybersecurity Fundamentals"
date = 2025-07-05
description = "A comprehensive introduction to information security fundamentals, covering the CIA triad, Common Criteria evaluation standards, OWASP Top 10 vulnerabilities, and practical security practices for modern applications."

[taxonomies]
tags = ["security", "technology", "standards"]
+++

Security - whether information security or physical - exists at the intersection of asset valuation, threat modeling, and risk management. To understand information security as a domain, let us begin by introducing its fundamental concepts - minimum necessary prerequisites and general context.

## Fundamentals of Security

- **Assets** - Objects of protection requiring security measures. These manifest across multiple domains: physical (hardware infrastructure, facilities), informational (data at rest, data in transit, intellectual property), and reputational (brand trust, market confidence).
- Assets are subject to **Threats** - Sources of potential harm, characterized by capability, intent, and opportunity. These range from non-directed environmental hazards to sophisticated Advanced Persistent Threats (APTs) with nation-state backing.
- Threats manifest through **Vulnerabilities** - Exploitable weaknesses in assets or their protective controls. These may be inherent to the asset's design, emergent from complex system interactions, or introduced through flawed operational processes.
- Vulnerabilities are exploitable in **Attacks** - The actualization of threats through vulnerability exploitation, resulting in compromise of security objectives.

Security, therefore, is a practice of identifying assets, understanding the threats and vulnerabilities, and putting up defenses (controls or countermeasures) to prevent attacks.

Organizations employ various categories controls to implement security:

- **Technical** - hardware and software mechanisms to protect assets, such as firewalls, encryption, or key card room access;
- **Managerial** - policies and guidelines that direct the security program and day-to-day operations to keep risks in check;
- **Operational** - supportive procedures, such as security awareness training, incident response plans, and backup and recovery procedures;
- **Physical** - locks, fences, security guards, and environmental controls.

These categories can be further typed by their function:

- **Preventative** - to stop incidents before they happen, like firewalls and access control lists;
- **Deterrent** - to discourage potential attackers, like warnings or visible presence of security cameras;
- **Corrective** - to remediate issues and restore systems to their normal state, like restoring backups or patching vulnerabilities;
- **Compensating** - alternative measures for cases when failure of primary control is expected, such as increased monitoring when a system cannot be patched;
- **Directive** - administrative security policies, such as acceptable use policy.

### CIA triad

The strategic objectives of any information security program are articulated through the **CIA triad** - **Confidentiality**, **Integrity**, and **Availability**, are three principles representing the fundamental goals that security infrastructure strives to achieve.

**Confidentiality** implements the concealment of information and resources, ensuring that sensitive data remains accessible only to authorized parties. This principle extends beyond simple secrecy to encompass sophisticated access control mechanisms based on the principles of least privilege and need-to-know. The implementation of confidentiality relies heavily on cryptographic controls, particularly encryption for data-in-transit (protecting information as it moves across networks) and data-at-rest (securing stored information). Modern confidentiality measures must also address inference attacks, where seemingly innocuous data can be combined to reveal sensitive information, and/or covert channels for information leakage.

> **Encryption** is the process of scrambling plaintext (readable data) into ciphertext (unreadable data) using an algorithm _with a trapdor_ such that the result cannot be reversed into plaintext without a correct _key_. In **symmetric encryption**, the same key can be used to both encrypt and decrypt data, while in **asymmetric encryption**, a key pair is used: a _public key_, which can be shared and used to encrypt data, and a _private key_, which is kept in secrecy by recipient of encrypted data so that only the recipient can "unscramble" the data.

**Integrity** concerns the trustworthiness of data and systems, manifesting in two distinct but related forms. Data integrity ensures that information cannot undergo unauthorized modification, whether through malicious action or accidental corruption. System integrity guarantees that computational systems perform their intended functions without unauthorized alterations to their behavior. Cryptographic hash functions serve as the mathematical foundation for verifying data integrity, producing unique fingerprints that reveal any modifications. Beyond technical controls, integrity relies on comprehensive change management processes, audit trails, and version control systems that create an immutable record of all modifications.

> A **hash** is a fixed-fize number, typically encoded in hexadecimal string, generated by a unidirectional mathematical function called a _hash algorithm_ (like SHA-256). Such a function takes input of arbitrary size and produces a unique, fixed-length output, such that even a miniscule change to the initial data will result in a completely dufferent hash. This supplements integrity verification because presence of errors or changes in data would cause easily identifiable change in its hash.

**Availability** ensures that systems and data remain reliably accessible to authorized users when needed. This principle confronts both accidental failures—hardware malfunctions, software bugs, or human errors—and deliberate attacks such as Denial-of-Service (DoS) campaigns. Achieving high availability requires architectural decisions including redundancy at multiple levels (from RAID storage to geographically distributed data centers), load balancing to distribute requests across multiple servers, and comprehensive disaster recovery planning that addresses both immediate failover and long-term business continuity.

The relationship among these three principles reveals fundamental tensions in security design. Enhanced confidentiality through encryption can compromise availability if cryptographic keys become inaccessible, effectively rendering data permanently unreadable. Aggressive availability measures, such as widespread data replication, multiply the attack surface for confidentiality breaches and create synchronization challenges for maintaining integrity. Similarly, stringent integrity controls through digital signatures and verification processes can introduce latency that degrades availability. These tensions cannot be eliminated but must be carefully balanced based on the specific risk profile and operational requirements of each system.

The CIA triad also exhibits interesting emergent properties when considered holistically. For instance, the concept of "non-repudiation" - ensuring that parties cannot deny their actions - emerges from the intersection of integrity (proving data hasn't been altered) and confidentiality (protecting the cryptographic credentials that establish identity). Similarly, "authenticity" arises from combining all three principles: confidentiality of authentication credentials, integrity of identity assertions, and availability of authentication services.

### CIANA pentad

It is not uncommon to also see **CIANA** pentad or pentagon, which expands the CIA triad by elevating two critical components - **Non-Repudiation** and **Authenticity** - as core principles. When dealing with security incidents which already manifest, having track of provenance is paramount during forensic analysis, and in itself is deterring for many risks.

**Non-Repudiation** provides assurance that an actor cannot later falsely deny having performed an action, which provides legally admissible proof of origin and receipt, making digital systems as binding as their physical counterparts. The primary technical control here is the use of *digital signatures*, which cryptographically bind an individual's identity to data.

> One can think about digital signatures as a partial case of encryption in asymmetric cryptography, where the hash of authentic data is encrypted with _private_ key and not public key ("signed"), and attached to the file. A block of data signed with a private key would be decryptable by anyone with the associated public key, and would produce a hash of original data - but only one party can produce a valid signature for any given hash such that "decrypting" with public key would yield the original intended hash. Given trust in possession of correct public key, this provides with ability to verify that received data was not modified - by computing its hash and comparing to one contained in signature.

**Authenticity** is the guarantee that data, communications, and parties involved, are gneuine and not counterfeit or impersonated. This is closely related to integrity, but in addition to ensuring that data has not been altered, authenticity also uses *digital signatures* to confirm data origin. A message with attached plaintext hash could have perfect integrity, but lack authenticity if it was sent by an imposter. Authenticity is achieved through a variety of mechanisms, which in addition to signing include message authentication codes, and require sophisticated key infrastructure management to bind public keys to verifiable real-world identities via Certificate Authorities (CAs) which are used to build a chain of trust by signing other public keys as authenticated data, assuring users that they are interacting with legitimate entities when they visit HTTPS websites.

### Triple A's and Zero Trust

One more important mnemonic to know that lays foundation of modern software security is the **Triple A's**.

- **Authentication** should always be the first step, verifying the identity of a user or system, through passwords (something that the user *knows*), biometrics (something that the user *is*), and security tokens (something that the user *has*) - preferably, no less than two out of three (multi-factor authentication).
- **Authorization** determines the specific actions and resources the verified user is permitted to access, by checking access control lists (ACLs) and permissions, ensuring users only have access to what they need.
- **Accounting** (or Auditing) meticulously tracks user acitivities and resource consumption, creating a detailed log for security audits, billing, and forensic analysis in the event of security incident.

One can argue that a rigorous implementation of mentioned A's would constitute a successful implementation of Zero Trust Model, built on the principle of "never trust, always verify". While the term "zero trust" is subject of some controversy in the domain for its excessive use by "marketing and effective management people", it is nonetheless a solid and clear idea, to operate on the assumption that threats exist both inside and outside the network, no user or system should be trusted by default, regardless of their location and position.

Nonetheless, when aiming to implement Zero Trust Model, an IT infrastructure would often converge to a clear separation of the **control plane** from the **data plane**. The *control plane* is where access decision are made; it focuses on adaptive identity and context awareness to reduce the potential threat scope, implements policy-driven access controls, and creates secured zones to isolate resources. The *data plane* is where these policies are applied and enforced - it consists of the subject or system requesting access, a policy engine that makes the access decision based on the control plane's policies, a policy administrator that pushes policies to the enforcement points, and the policy enforcement points themselves, which grant or deny access to the data. This continuous verification and granular access control significantly enhances an organization's security posture and automatically ensures compliance, when implemented systematically and universally.

## Security Assessment Criteria

To evaluate whether a system meets the principles of the CIA triad, a formal methodology is required. The **Common Criteria for Information Technology Security Evaluation** (abbreviated as Common Criteria or CC, and standardized as ISO/IEC 15408) provides a globally recognized framework for specifying and evaluating security properties of IT products. The goal of CC is to provide a level of confidence that a product's claimed security features have been thoroughly and impartially evaluated.

The CC evaluation process uses a specific vocabulary. The product or system being evaluated is the **Target of Evaluation (ToE)**. Developers make claims about their ToE's security properties in a **Security Target (ST)** document. This ST may conform to a **Protection Profile (PP)**, which is a reusable, implementation-independent document that defines a standard set of security requirements for a whole class of products (e.g., firewalls or operating systems).

A central concept in the Common Criteria is the **Evaluation Assurance Level (EAL)**, a numerical rating from EAL1 to EAL7 that describes the rigor and depth of the evaluation. A higher EAL does not imply more security features; rather, it signifies that the product's security claims have been verified with a greater degree of assurance.

- EAL1: Functionally Tested
- EAL2: Structurally Tested
- EAL3: Methodically Tested and Checked
- EAL4: Methodically Designed, Tested, and Reviewed
- EAL5: Semi-formally Designed and Tested
- EAL6: Semi-formally Verified Design and Tested
- EAL7: Formally Verified Design and Tested

This progression is not merely about testing, but also about formal proof of correctness, moving from basic functional testing at EAL1 to formal, mathematical design verification at EAL7.

The specific security behaviors claimed in a Security Target are defined using **Security Functional Requirements (SFRs)**. These are drawn from a standardized catalog and describe *what* a product must do to be considered secure. An ST will select a specific set of SFRs to define the exact security functions of the ToE. These requirements are grouped into families, including:

- FAU (Security Audit) - transparency of system operations for violation analysis and anomaly detection.
- FCO (Communication) - party identification and non-repudiation.
- FCS (Cryptographic Support) - use of cryptography; key management lifecycle.
- FDP (User Data Protection) - user data protection, including access control and information flow control.
- FIA (Identity and Authentication) - user identity verification before granting access to ToE.
- FMT (Security Management) - roles, security attributes, access level, and function management.
- FPR (Privacy) - measures for protecting user privacy; anonymity, pseudonymity, unlinkability and unobservability.
- FPT (Protection of the ToE Security functions) - protection of TOE's own security mechanisms from tampering or bypass; their integrity, secure recovery, and self-testing.
- FRU (Resource Utilisation) - protection against resource exhaustion attacks like Denial-of-Service; fault tolerance and quality-of-service.
- FTA (TOE Access) - access control to the TOE as a whole, session management.
- FTP (Trusted Path/Channels) - security of communications between the user and the TOE or between components of the TOE from modification or interception.

Complementing the SFRs are the **Security Assurance Requirements (SARs)**, which define *how* confidence in the correct implementation of the SFRs is achieved. SARs dictate the actions an evaluator must perform to verify the security claims. These are also grouped into families:

- ADV (Development) - analysis of the design documentation, or the source code itself for higher EALs.
- AGD (Guidance Documents) - analysis of administrator and user documentation; clarity of guidance on installation, configuration, and operation of the product in a secure manner.
- ALC (Life-cycle Support) - assurance that the product is developed and maintained in a secure environment; requirements for developer's configuration management system and security measures, including physical security and personnel screening, and procedures for flaw remediation and secure delivery
- ASE (Security Target Evaluation) - listing of security properties, to ensure the ST is complete, consistent, and sound.
- ATE (Tests) - verification developer's functional tests and/or conducting independent testing.
- AVA (Vulnerability assessment) - analysis of threat vectors and potential vulnerabilities, with AVA_VAN subclass establishing rigorous analysis from checking for publicly known vulnerabilities at low levels to sophisticated penetration testing at higher levels.
- ACO (Composition) - assurance that the TOE composed of multiple components is secure as a whole.
- APE (Protection Profile Evaluation) - set of requirements for certain class of products, defining actions to evaluate a PP to ensure it is complete, consistent, and useful.

### European Union

Within the European Union, the recognition of formal security evaluations has a long history. Initially, European nations established the **SOGIS (Senior Officials Group Information Systems Security) Mutual Recognition Agreement (MRA)**. This agreement was significant as it allowed for the mutual recognition of high-assurance Common Criteria certificates (up to EAL7) among its member states, reducing redundant evaluations for vendors.

More recently, the EU has moved to further harmonize the cybersecurity landscape with the introduction of the **Cybersecurity Act (CSA)** in 2019. This act establishes a pan-European certification framework managed by **ENISA (The European Union Agency for Cybersecurity)**. A key outcome of the CSA is the **EUCC (EU Common Criteria)** scheme, which builds upon the foundations of CC and SOGIS but introduces a single, harmonized approach for all member states. A notable feature of the EUCC is its increased emphasis on the vulnerability assessment level (AVA_VAN), which defines the required resistance of a ToE against attackers with specific capabilities. While certification under the CSA is generally voluntary, individual EU nations can mandate it for critical sectors such as public services, essential infrastructure, or specific high-risk devices.

## Application Security

Common Criteria provide formal framework for evaluation of security which is mostly a kind of bureaucratic framework, but what about practical security for common web applications? The most authoritative guidance in this domain comes from the **OWASP (Open Web Application Security Project) Top 10**. Rather than a formal standard, the OWASP Top 10 is a consensus-driven awareness document that identifies the most critical and prevalent security risks facing web applications. It serves as a vital tool for developers, architects, and defenders to prioritize their mitigation efforts.

**Note:** The [OWASP Cheat Sheet Series](https://cheatsheetseries.owasp.org/) provides concise, actionable guidance on specific application security topics.

### A01:2021 Broken Access Control

Broken Access Control encompasses a class of vulnerabilities where users can gain access to data or perform actions beyond their intended permissions. These failures arise when an application's restrictions on authenticated users are not properly enforced, often due to logical flaws in the access control mechanism. A critical mistake is performing security checks on the client-side (e.g., in the user's browser), as such checks can be trivially bypassed. **Access control decisions must be made and enforced on the server.**

A common manifestation is **Insecure Direct Object Referencing (IDOR)**, where an application exposes a direct reference to an internal implementation object, such as a file or database key. For instance, if a URL like `https://example.com/data?invoice_id=123` is used to retrieve a document, an attacker could manipulate the `invoice_id` parameter to access invoices belonging to other users, assuming the server fails to verify ownership for each request.

A more severe example is **Privilege Esalation** - either vertical (elevation to administrator and access to admin-only resources) or horizontal (assuming access to the data or functions of another user, similar to previous example). The key to mitigating most A01 vulnerabilities is to never trust input provided from clients and enforce access control on the server-side with every single client request.

### A02:2021 Cryptographic Failures

This category addresses failures in protecting data, which can compromise both Confidentiality and Integrity. Such failures often serve as a root cause for other vulnerabilities; for instance, if cryptographic tokens used for authentication are compromised, it can lead directly to Broken Access Control (A01). The A02 category focuses on the protection of data *in transit* (moving across a network) and *at rest* (stored in a database, on disk, or on a client device).

Common failures include:
- **Transmitting data in cleartext:** For example, using HTTP instead of HTTPS for login forms or other sensitive data exchange, making it vulnerable to eavesdropping.
- **Using weak or outdated cryptographic algorithms:** Employing algorithms with known vulnerabilities, such as the MD5 hash function or the RC4 stream cipher, which can be broken with modern computational resources.
- **Improper key management:** This is a critical and frequent failure point, encompassing issues like hard-coding cryptographic keys in source code, using weak or default keys, or failing to implement a key rotation policy.
- **Failing to validate token integrity:** Forgetting to verify the cryptographic signature on a JSON Web Token (JWT), for example, allows an attacker to modify the token's contents and potentially impersonate other users or elevate their privileges.
- **Storing sensitive data without encryption:** Persisting user passwords, personal information, or session tokens in a non-encrypted or weakly hashed format.

A robust security posture demands strong, industry-standard encryption for all sensitive data, both in transit and at rest, coupled with a secure key management lifecycle.

### A03:2021 Injection

Injection flaws are a broad class of vulnerabilities where an attacker can supply untrusted input to an application that is then relayed to an interpreter as part of a command or query. The core failure is the application's inability to separate untrusted user data from the intended command structure, allowing the interpreter to execute the malicious input.

The most well-known example is **SQL Injection (SQLi)**. Consider an application that constructs a database query via string concatenation: `SELECT * FROM users WHERE username = '" + userName + "';`. If the application fails to sanitize the `userName` input, an attacker can submit a malicious payload like `' OR '1'='1`. The resulting query becomes `SELECT * FROM users WHERE username = '' OR '1'='1';`. The `OR '1'='1'` clause forces the `WHERE` condition to be true for every row, causing the database to return all users and effectively bypassing authentication.

The definitive mitigation for SQLi is the use of **parameterized queries**, also known as prepared statements. This approach separates the query structure from the user-supplied data. The application first sends the query template with placeholders (e.g., `SELECT * FROM users WHERE username = ?;`) to the database engine. Then, it sends the user's input as a separate parameter. The database engine treats this input strictly as data and never as executable code, thus neutralizing the injection attack.

While SQLi is common, injection is a broader problem that also includes **Cross-Site Scripting (XSS)**, where malicious scripts are injected into web pages and executed in victims' browsers, as well as injections into OS command interpreters, LDAP, or NoSQL databases.

### A04:2021 Insecure Design

Insecure Design is a broad category representing weaknesses that stem from a failure to incorporate security thinking into the application architecture from its inception. Unlike implementation bugs, these are flaws in the design itself, which can be costly and difficult to remediate later in the development lifecycle.

- Lack of Threat Modeling. The core approach to investigating security of a design is by questioning design through a structured process of identifying potential threats and vulnerabilities. For example, suppose there is a "password reset" features which sends a reset link to the user's email. A secure design would require answering a secret question or sending a code to a trusted device _before_ the email is even sent, otherwise the security of the account reduces to that of the email inbox. Indeed, many services do not bother with considering such threat to be meaningful, allowing phishing to flourish today.
- Flawed Business Logic. The application might function reliably, but the workflow itself might be exploitable. This sometimes happens to small e-shops: if the system checks the price only when you add an item to the cart, you might add it to a cart during the sale, and then still buy it at that price after the sale expires - allowing user to buy items at a lower, expired sale price.
- Not Planning for Security Controls. The application design has to account for the need for separation of privileges, rate limiting, or other security controls, early on. If an internal admin application is built with the assumptions that all administrators are equally trusted, it might be harder to introduce fine-grained control - and if a junior administrator's account is compromised, the attacker might gain full control of the entire system because the design never planned for different levels of administrative access.

This is a pretty common class despite its genericity, which requires a mitigation that extends beyond technological means - the design of application has to "shift left", moving security to the earliest possible point in the development lifecycle. Companies are obliged to define Secure Development Lifecycle in their strategic documents and integrate security practices like threat modeling and architecture reviews into every development stage, from initial idea to deployment and maintenance.

For regular evaluation of new features and architectures, a common, relatively simple and effective way is to ask a number of question - according to a model abbreviated as STRIDE:
1. Spoofing: can an attacker pretend to be another user?
2. Tampering: can they modify data in transit or at rest?
3. Repudiation: could a user deny having performed an action?
4. Information Disclosure: can user access data it should not?
5. Denial of Service: can user crash or disable the system for others?
6. Elevation of Privilege: can a low-privilege user gain administrative rights?

### A05:2021 Security Misconfiguration

The application might have the strongest walls, good code, and best alarm systems and observability, but if the front door is open because of a configuration mistake, none of that matters. In operations, when security settings are defined, implemented, or maintained incorrectly, it is often the result of using insecure defaults, leaving temporary configurations in place. This category of vulnerabilities is commonly said to be all about human errors and lack of attention to detail during setup and maintenance - but I would argue that sensible defaults and clear heuristic warnings within applications are paramount for mitigating such issues.

- **Default Credentials:** Many systems, from network hardware to databases, ship with well-known default credentials (e.g., `admin/admin`). Failing to change these immediately upon deployment provides an easy entry point for attackers.
- **Verbose Error Messages:** Error pages that reveal excessive detail—such as full stack traces, internal file paths, or database schema information—can provide attackers with a roadmap of the system's internal architecture, aiding in further attacks.
- **Unnecessary Services:** Exposing non-production features or running unnecessary services on a production server increases the attack surface, providing more potential vectors for compromise.
- **Improper Environment Separation:** A classic misconfiguration is a web server configured to list the contents of directories. This can expose sensitive source code or configuration files (like `.git` directories). This often points to a deeper issue: a failure to use a hardened build and deployment process that ensures only necessary, sanitized artifacts are deployed to the production environment.

### A06:2021 Vulnerable and Outdated Components

Much like in real logistics, software delivery process has its own _supply chain_ which you might not always be able to trust. Modern applications and server operating systems often involve hundreds of third-party components - packages, libraries and frameworks. It means the application inherits the security posture of every one of those components - and if a library you rely on, directly or indirectly, has a security flaw, the application will have that same flaw.

A particularly huge example was a severe remote code execution (RCE) vulnerability found in Log4j, an extremely common Java logging library, exposing millions of applications to an injection vulnerability where an attacker could get a server to run their code simply by getting the application to log a malicious string. Truly catastrophic.

To avoid being targeted, application developers and operators must maintain an inventory of critical components and control which components can be added and removed, and try to stay informed by subscribing to security bulletins. Lately, there has been a plethora of automated scanning tools to detect vulnerable components, either using means built into package managers of modern programming languages, or using commercial platforms for Software Composition Analysis (SCA) to automatically find vulnerable dependencies and even suggest fixes to them with automatically submitted pull requests.

In addition, it is worth to have clarity on the process of testing and deploying updates when a vulnerability is announces by maintaining an internal patching policy. The goal is to apply security patches quickly while ensuring they do not break the application.

### A07:2021 Identification and Authentication Failures

Authentication is the process of verifying the identity of a user or system. When authentication mechanisms are weak or broken, the entire security architecture of the application is compromised. The failures in this category historically were called "Broken Authentication" and include anything from compromised passwords to issues with multi-factor authentication.

- **Weak Password Policies:** Allowing or requiring weak passwords, such as "password123" or "admin", makes accounts vulnerable to brute-force attacks or dictionary attacks.
- **Credential Stuffing:** This occurs when users reuse passwords across multiple services. If one service is compromised, attackers can use the leaked credentials to access other services.
- **Session Management Issues:** Improper session handling can lead to session hijacking, where an attacker can steal or predict a user's session token and impersonate them.
- **Insecure Password Recovery:** Weak password reset mechanisms that rely on predictable questions or email confirmation without proper verification can be exploited.
- **Multi-Factor Authentication (MFA) Bypass:** Flaws in MFA implementation can allow attackers to bypass the additional security layer, such as accepting SMS codes that were never sent or allowing users to skip MFA under certain conditions.

Strong authentication mechanisms include implementing robust password policies, using multi-factor authentication, protecting against brute-force attacks through rate limiting, and ensuring secure session management with proper timeout and invalidation procedures.

### A08:2021 Software and Data Integrity Failures

This category addresses failures in code and infrastructure that do not protect against integrity violations. This includes scenarios where an application relies on plugins, libraries, or modules from untrusted sources, repositories, and content delivery networks (CDNs) without proper verification.

- **Unsigned or Improperly Signed Code:** Applications that automatically update from untrusted sources without verifying the integrity of the code can be compromised. For example, if a JavaScript library is served from a CDN without proper integrity checks, an attacker who compromises the CDN can inject malicious code into the application.
- **Insecure CI/CD Pipelines:** Compromised build and deployment pipelines can inject malicious code into applications. This is particularly dangerous because the malicious code appears to come from a trusted source.
- **Deserialisation Vulnerabilities:** Applications that deserialize untrusted data without proper validation can execute arbitrary code. This is particularly common in languages like Java and Python, where serialized objects can contain executable code.

To mitigate these risks, applications should verify the integrity of all code and data, use secure CI/CD pipelines with proper access controls, and avoid deserializing untrusted data without proper validation.

### A09:2021 Security Logging and Monitoring Failures

Security logging and monitoring is crucial for detecting and responding to security incidents. However, many applications have insufficient logging, making it difficult to detect breaches or understand the scope of an incident.

- **Insufficient Logging:** Applications that don't log security-relevant events, such as login attempts, access control failures, or data access, make it difficult to detect attacks or investigate incidents.
- **Logs Not Monitored:** Even if logs are generated, they must be actively monitored. Logs that are generated but never reviewed are of little value.
- **Ineffective Incident Response:** Without proper logging and monitoring, organizations cannot effectively respond to security incidents, leading to prolonged exposure and greater damage.

Effective security logging should capture all security-relevant events, be tamper-resistant, and be actively monitored. Organizations should have incident response procedures in place and regular testing of their detection and response capabilities.

### A10:2021 Server-Side Request Forgery (SSRF)

Server-Side Request Forgery (SSRF) vulnerabilities occur when a web application fetches resources from URLs provided by users without proper validation. This allows attackers to force the server to make requests to unintended destinations, potentially exposing internal systems or services that are not directly accessible from the internet.

- **Internal Network Scanning:** Attackers can use SSRF to scan internal networks and discover services that are not exposed to the internet.
- **Accessing Internal Services:** SSRF can be used to access internal services like databases, admin panels, or cloud metadata services that assume trust from internal requests.
- **Cloud Metadata Exploitation:** In cloud environments, SSRF can be used to access metadata services that provide sensitive information like access keys or instance details.

To prevent SSRF vulnerabilities, applications should validate and sanitize all URLs before making requests, use allowlists for permitted domains, and implement network segmentation to limit the impact of successful attacks.

## Conclusion

These fundamentals provide a foundation for understanding information security, but they are just the beginning. The field is continuously evolving, with new threats and vulnerabilities emerging regularly. Staying informed about current threats, best practices, and new technologies is essential for maintaining a strong security posture.

The key to effective information security is not just understanding these concepts, but applying them systematically throughout the development and deployment lifecycle. Security is not a one-time consideration but an ongoing process that requires continuous attention and improvement.
