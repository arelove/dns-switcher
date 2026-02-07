<script lang="ts">
  interface Props {
    dnsName: string;
    icon: string;
    color: string;
  }
  
  let { dnsName, icon, color }: Props = $props();
</script>

<div class="status-section">
  <div class="status-icon" style="background: {color}20;">
    <span style="font-size: 2rem;">{icon}</span>
  </div>
  <div class="status-info">
    <div class="status-label">Active DNS Service</div>
    <div class="status-name" style="color: {color};">
      {dnsName}
    </div>
  </div>
</div>

<style>
  .status-section {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 1rem;
    background: rgba(255, 255, 255, 0.5);
    border-radius: 12px;
  }
  
  @media (prefers-color-scheme: dark) {
    .status-section {
      background: rgba(0, 0, 0, 0.2);
    }
  }
  
  .status-icon {
    width: 4rem;
    height: 4rem;
    border-radius: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .status-info {
    flex: 1;
  }
  
  .status-label {
    font-size: 0.75rem;
    font-weight: 600;
    color: #718096;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: 0.25rem;
  }
  
  @media (prefers-color-scheme: dark) {
    .status-label {
      color: #cbd5e0;
    }
  }
  
  .status-name {
    font-size: 1.25rem;
    font-weight: 700;
  }
</style>