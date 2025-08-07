# Kosher Chain Governance Model

The integrity and purpose of the Kosher Chain are maintained through a robust, off-chain governance model managed by a Rabbinic Council. This council is the ultimate authority on the network's rules and its participants.

## The Rabbinic Council

The Council is comprised of respected Rabbinic authorities and Halachic scholars who are experts in the principles the chain is founded upon. They are responsible for:

1.  **Defining On-Chain Rules:** Interpreting and setting the Halachic policies that are encoded into the chain's logic (e.g., rules for Shabbat, permitted types of commerce).
2.  **Validator Vetting & Approval:** Approving and formally appointing all validator operators.
3.  **Dispute Resolution:** Serving as the final arbiter for any disputes that may arise on the network.

## Validator Management

Validators are the technical operators of the network. The process for adding or removing a validator is strictly governed.

### Adding a Validator

1.  **Nomination:** A candidate organization or individual must be nominated to the Council.
2.  **Vetting:** The Council performs a thorough off-chain review of the candidate to ensure they are aligned with the project's mission and have the technical competence to operate a secure node.
3.  **Approval:** If approved, the Council issues a formal, signed declaration of the new validator's status.
4.  **Technical Onboarding:** The new validator's public key is added to the `validators.json` file in a coordinated network update.

### Removing a Validator

A validator can be removed for technical reasons (e.g., prolonged downtime) or for violating the network's Halachic principles. The process requires a majority vote from the Council, after which their public key is removed from the validator set in a subsequent network update.

## Network Upgrades

All significant changes to the node software, consensus rules, or core smart contracts must be audited and subsequently approved by the Council before being deployed.
