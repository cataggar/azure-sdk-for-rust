// Copyright (c) Microsoft Corporation. All rights reserved.
//
// Licensed under the MIT License. See License.txt in the project root for license information.
// Code generated by Microsoft (R) Rust Code Generator. DO NOT EDIT.

use crate::models::{
    Addon, AddonList, AdminCredentials, CloudLink, CloudLinkList, Cluster, ClusterList,
    ClusterUpdate, ClusterZoneList, Datastore, DatastoreList, ExpressRouteAuthorization,
    ExpressRouteAuthorizationList, GlobalReachConnection, GlobalReachConnectionList,
    HcxEnterpriseSite, HcxEnterpriseSiteList, IscsiPath, IscsiPathListResult, OperationListResult,
    PlacementPoliciesList, PlacementPolicy, PlacementPolicyUpdate, PrivateCloud, PrivateCloudList,
    PrivateCloudUpdate, Quota, ScriptCmdlet, ScriptCmdletsList, ScriptExecution,
    ScriptExecutionsList, ScriptPackage, ScriptPackagesList, Sku, Trial, VirtualMachine,
    VirtualMachineRestrictMovement, VirtualMachinesList, WorkloadNetwork, WorkloadNetworkDhcp,
    WorkloadNetworkDhcpList, WorkloadNetworkDnsService, WorkloadNetworkDnsServicesList,
    WorkloadNetworkDnsZone, WorkloadNetworkDnsZonesList, WorkloadNetworkGateway,
    WorkloadNetworkGatewayList, WorkloadNetworkList, WorkloadNetworkPortMirroring,
    WorkloadNetworkPortMirroringList, WorkloadNetworkPublicIP, WorkloadNetworkPublicIPsList,
    WorkloadNetworkSegment, WorkloadNetworkSegmentsList, WorkloadNetworkVMGroup,
    WorkloadNetworkVMGroupsList, WorkloadNetworkVirtualMachine, WorkloadNetworkVirtualMachinesList,
};
use async_std::task::block_on;
use azure_core::{RequestContent, Response, Result};
use typespec_client_core::json::to_json;

impl TryFrom<Addon> for RequestContent<Addon> {
    type Error = azure_core::Error;
    fn try_from(value: Addon) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<Addon>> for Addon {
    type Error = azure_core::Error;
    fn try_from(value: Response<Addon>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<AddonList>> for AddonList {
    type Error = azure_core::Error;
    fn try_from(value: Response<AddonList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<AdminCredentials>> for AdminCredentials {
    type Error = azure_core::Error;
    fn try_from(value: Response<AdminCredentials>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<CloudLink> for RequestContent<CloudLink> {
    type Error = azure_core::Error;
    fn try_from(value: CloudLink) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<CloudLink>> for CloudLink {
    type Error = azure_core::Error;
    fn try_from(value: Response<CloudLink>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<CloudLinkList>> for CloudLinkList {
    type Error = azure_core::Error;
    fn try_from(value: Response<CloudLinkList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Cluster> for RequestContent<Cluster> {
    type Error = azure_core::Error;
    fn try_from(value: Cluster) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<Cluster>> for Cluster {
    type Error = azure_core::Error;
    fn try_from(value: Response<Cluster>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<ClusterList>> for ClusterList {
    type Error = azure_core::Error;
    fn try_from(value: Response<ClusterList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<ClusterUpdate> for RequestContent<ClusterUpdate> {
    type Error = azure_core::Error;
    fn try_from(value: ClusterUpdate) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<ClusterZoneList>> for ClusterZoneList {
    type Error = azure_core::Error;
    fn try_from(value: Response<ClusterZoneList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Datastore> for RequestContent<Datastore> {
    type Error = azure_core::Error;
    fn try_from(value: Datastore) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<Datastore>> for Datastore {
    type Error = azure_core::Error;
    fn try_from(value: Response<Datastore>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<DatastoreList>> for DatastoreList {
    type Error = azure_core::Error;
    fn try_from(value: Response<DatastoreList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<ExpressRouteAuthorization> for RequestContent<ExpressRouteAuthorization> {
    type Error = azure_core::Error;
    fn try_from(value: ExpressRouteAuthorization) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<ExpressRouteAuthorization>> for ExpressRouteAuthorization {
    type Error = azure_core::Error;
    fn try_from(value: Response<ExpressRouteAuthorization>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<ExpressRouteAuthorizationList>> for ExpressRouteAuthorizationList {
    type Error = azure_core::Error;
    fn try_from(value: Response<ExpressRouteAuthorizationList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<GlobalReachConnection> for RequestContent<GlobalReachConnection> {
    type Error = azure_core::Error;
    fn try_from(value: GlobalReachConnection) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<GlobalReachConnection>> for GlobalReachConnection {
    type Error = azure_core::Error;
    fn try_from(value: Response<GlobalReachConnection>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<GlobalReachConnectionList>> for GlobalReachConnectionList {
    type Error = azure_core::Error;
    fn try_from(value: Response<GlobalReachConnectionList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<HcxEnterpriseSite> for RequestContent<HcxEnterpriseSite> {
    type Error = azure_core::Error;
    fn try_from(value: HcxEnterpriseSite) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<HcxEnterpriseSite>> for HcxEnterpriseSite {
    type Error = azure_core::Error;
    fn try_from(value: Response<HcxEnterpriseSite>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<HcxEnterpriseSiteList>> for HcxEnterpriseSiteList {
    type Error = azure_core::Error;
    fn try_from(value: Response<HcxEnterpriseSiteList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<IscsiPath> for RequestContent<IscsiPath> {
    type Error = azure_core::Error;
    fn try_from(value: IscsiPath) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<IscsiPath>> for IscsiPath {
    type Error = azure_core::Error;
    fn try_from(value: Response<IscsiPath>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<IscsiPathListResult>> for IscsiPathListResult {
    type Error = azure_core::Error;
    fn try_from(value: Response<IscsiPathListResult>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<OperationListResult>> for OperationListResult {
    type Error = azure_core::Error;
    fn try_from(value: Response<OperationListResult>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<PlacementPoliciesList>> for PlacementPoliciesList {
    type Error = azure_core::Error;
    fn try_from(value: Response<PlacementPoliciesList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<PlacementPolicy> for RequestContent<PlacementPolicy> {
    type Error = azure_core::Error;
    fn try_from(value: PlacementPolicy) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<PlacementPolicy>> for PlacementPolicy {
    type Error = azure_core::Error;
    fn try_from(value: Response<PlacementPolicy>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<PlacementPolicyUpdate> for RequestContent<PlacementPolicyUpdate> {
    type Error = azure_core::Error;
    fn try_from(value: PlacementPolicyUpdate) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<PrivateCloud> for RequestContent<PrivateCloud> {
    type Error = azure_core::Error;
    fn try_from(value: PrivateCloud) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<PrivateCloud>> for PrivateCloud {
    type Error = azure_core::Error;
    fn try_from(value: Response<PrivateCloud>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<PrivateCloudList>> for PrivateCloudList {
    type Error = azure_core::Error;
    fn try_from(value: Response<PrivateCloudList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<PrivateCloudUpdate> for RequestContent<PrivateCloudUpdate> {
    type Error = azure_core::Error;
    fn try_from(value: PrivateCloudUpdate) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<Quota>> for Quota {
    type Error = azure_core::Error;
    fn try_from(value: Response<Quota>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<ScriptCmdlet>> for ScriptCmdlet {
    type Error = azure_core::Error;
    fn try_from(value: Response<ScriptCmdlet>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<ScriptCmdletsList>> for ScriptCmdletsList {
    type Error = azure_core::Error;
    fn try_from(value: Response<ScriptCmdletsList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<ScriptExecution> for RequestContent<ScriptExecution> {
    type Error = azure_core::Error;
    fn try_from(value: ScriptExecution) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<ScriptExecution>> for ScriptExecution {
    type Error = azure_core::Error;
    fn try_from(value: Response<ScriptExecution>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<ScriptExecutionsList>> for ScriptExecutionsList {
    type Error = azure_core::Error;
    fn try_from(value: Response<ScriptExecutionsList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<ScriptPackage>> for ScriptPackage {
    type Error = azure_core::Error;
    fn try_from(value: Response<ScriptPackage>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<ScriptPackagesList>> for ScriptPackagesList {
    type Error = azure_core::Error;
    fn try_from(value: Response<ScriptPackagesList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Sku> for RequestContent<Sku> {
    type Error = azure_core::Error;
    fn try_from(value: Sku) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<Trial>> for Trial {
    type Error = azure_core::Error;
    fn try_from(value: Response<Trial>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<VirtualMachine>> for VirtualMachine {
    type Error = azure_core::Error;
    fn try_from(value: Response<VirtualMachine>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<VirtualMachineRestrictMovement> for RequestContent<VirtualMachineRestrictMovement> {
    type Error = azure_core::Error;
    fn try_from(value: VirtualMachineRestrictMovement) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<VirtualMachinesList>> for VirtualMachinesList {
    type Error = azure_core::Error;
    fn try_from(value: Response<VirtualMachinesList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<WorkloadNetwork>> for WorkloadNetwork {
    type Error = azure_core::Error;
    fn try_from(value: Response<WorkloadNetwork>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<WorkloadNetworkDhcp> for RequestContent<WorkloadNetworkDhcp> {
    type Error = azure_core::Error;
    fn try_from(value: WorkloadNetworkDhcp) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<WorkloadNetworkDhcp>> for WorkloadNetworkDhcp {
    type Error = azure_core::Error;
    fn try_from(value: Response<WorkloadNetworkDhcp>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<WorkloadNetworkDhcpList>> for WorkloadNetworkDhcpList {
    type Error = azure_core::Error;
    fn try_from(value: Response<WorkloadNetworkDhcpList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<WorkloadNetworkDnsService> for RequestContent<WorkloadNetworkDnsService> {
    type Error = azure_core::Error;
    fn try_from(value: WorkloadNetworkDnsService) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<WorkloadNetworkDnsService>> for WorkloadNetworkDnsService {
    type Error = azure_core::Error;
    fn try_from(value: Response<WorkloadNetworkDnsService>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<WorkloadNetworkDnsServicesList>> for WorkloadNetworkDnsServicesList {
    type Error = azure_core::Error;
    fn try_from(value: Response<WorkloadNetworkDnsServicesList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<WorkloadNetworkDnsZone> for RequestContent<WorkloadNetworkDnsZone> {
    type Error = azure_core::Error;
    fn try_from(value: WorkloadNetworkDnsZone) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<WorkloadNetworkDnsZone>> for WorkloadNetworkDnsZone {
    type Error = azure_core::Error;
    fn try_from(value: Response<WorkloadNetworkDnsZone>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<WorkloadNetworkDnsZonesList>> for WorkloadNetworkDnsZonesList {
    type Error = azure_core::Error;
    fn try_from(value: Response<WorkloadNetworkDnsZonesList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<WorkloadNetworkGateway>> for WorkloadNetworkGateway {
    type Error = azure_core::Error;
    fn try_from(value: Response<WorkloadNetworkGateway>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<WorkloadNetworkGatewayList>> for WorkloadNetworkGatewayList {
    type Error = azure_core::Error;
    fn try_from(value: Response<WorkloadNetworkGatewayList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<WorkloadNetworkList>> for WorkloadNetworkList {
    type Error = azure_core::Error;
    fn try_from(value: Response<WorkloadNetworkList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<WorkloadNetworkPortMirroring> for RequestContent<WorkloadNetworkPortMirroring> {
    type Error = azure_core::Error;
    fn try_from(value: WorkloadNetworkPortMirroring) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<WorkloadNetworkPortMirroring>> for WorkloadNetworkPortMirroring {
    type Error = azure_core::Error;
    fn try_from(value: Response<WorkloadNetworkPortMirroring>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<WorkloadNetworkPortMirroringList>> for WorkloadNetworkPortMirroringList {
    type Error = azure_core::Error;
    fn try_from(value: Response<WorkloadNetworkPortMirroringList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<WorkloadNetworkPublicIP> for RequestContent<WorkloadNetworkPublicIP> {
    type Error = azure_core::Error;
    fn try_from(value: WorkloadNetworkPublicIP) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<WorkloadNetworkPublicIP>> for WorkloadNetworkPublicIP {
    type Error = azure_core::Error;
    fn try_from(value: Response<WorkloadNetworkPublicIP>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<WorkloadNetworkPublicIPsList>> for WorkloadNetworkPublicIPsList {
    type Error = azure_core::Error;
    fn try_from(value: Response<WorkloadNetworkPublicIPsList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<WorkloadNetworkSegment> for RequestContent<WorkloadNetworkSegment> {
    type Error = azure_core::Error;
    fn try_from(value: WorkloadNetworkSegment) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<WorkloadNetworkSegment>> for WorkloadNetworkSegment {
    type Error = azure_core::Error;
    fn try_from(value: Response<WorkloadNetworkSegment>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<WorkloadNetworkSegmentsList>> for WorkloadNetworkSegmentsList {
    type Error = azure_core::Error;
    fn try_from(value: Response<WorkloadNetworkSegmentsList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<WorkloadNetworkVMGroup> for RequestContent<WorkloadNetworkVMGroup> {
    type Error = azure_core::Error;
    fn try_from(value: WorkloadNetworkVMGroup) -> Result<Self> {
        RequestContent::try_from(to_json(&value)?)
    }
}

impl TryFrom<Response<WorkloadNetworkVMGroup>> for WorkloadNetworkVMGroup {
    type Error = azure_core::Error;
    fn try_from(value: Response<WorkloadNetworkVMGroup>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<WorkloadNetworkVMGroupsList>> for WorkloadNetworkVMGroupsList {
    type Error = azure_core::Error;
    fn try_from(value: Response<WorkloadNetworkVMGroupsList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<WorkloadNetworkVirtualMachine>> for WorkloadNetworkVirtualMachine {
    type Error = azure_core::Error;
    fn try_from(value: Response<WorkloadNetworkVirtualMachine>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}

impl TryFrom<Response<WorkloadNetworkVirtualMachinesList>> for WorkloadNetworkVirtualMachinesList {
    type Error = azure_core::Error;
    fn try_from(value: Response<WorkloadNetworkVirtualMachinesList>) -> Result<Self> {
        let f = || value.into_json_body();
        let r = block_on(f())?;
        Ok(r)
    }
}
