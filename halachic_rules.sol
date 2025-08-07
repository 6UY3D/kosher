// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

/**
 * @title HalachicRules
 * @dev An example contract to demonstrate on-chain governance rules.
 */
contract HalachicRules {
    address public rabbinicAuthority;
    bool public isPermitted;

    event AuthorityUpdated(address indexed newAuthority);
    event PermissionChanged(bool isPermitted);

    modifier onlyAuthority() {
        require(msg.sender == rabbinicAuthority, "Only the Rabbinic Authority can call this function");
        _;
    }

    constructor(address initialAuthority) {
        rabbinicAuthority = initialAuthority;
    }

    /**
     * @dev Sets the permission flag. Can only be called by the authority.
     */
    function setPermission(bool _isPermitted) public onlyAuthority {
        isPermitted = _isPermitted;
        emit PermissionChanged(_isPermitted);
    }

    /**
     * @dev Updates the governing authority address.
     */
    function updateAuthority(address newAuthority) public onlyAuthority {
        rabbinicAuthority = newAuthority;
        emit AuthorityUpdated(newAuthority);
    }
}
