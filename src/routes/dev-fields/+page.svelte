<div class="tile-item">
  <label class="tile-label" for="folder">Folder</label>
  <select class="tile-select" name="folder" id="folder">
    <option disabled value="">Loading...</option>
  </select>
  <p class="tile-description">Folder this tile will switch to when clicked</p>
</div>

<div class="tile-item">
  <label class="tile-label" for="name">Name</label>
  <input class="tile-input" type="text" id="name" placeholder="Example Name" />
  <p class="tile-description">Name of the tile</p>
</div>

<div class="tile-item">
  <label class="tile-label" for="enabled">Checkbox</label>

  <input
    class="tile-checkbox"
    type="checkbox"
    id="enabled"
    placeholder="Example Name"
  />

  <p class="tile-description">Should this tile be enabled</p>
</div>

<div class="tile-item">
  <button class="tile-button">Test</button>
</div>

<style>
  .tile-item {
    padding: 8px;
  }

  .tile-label {
    display: block;
    font-weight: 600;
    color: #ddd;
    font-weight: bold;
    margin-bottom: 2px;
  }

  .tile-description {
    display: block;
    font-size: 12px;
    color: #aaa;
    margin-top: 4px;
  }

  .tile-select {
    appearance: none;
    padding: 0.5rem 0.25rem;
    border: none;
    background-color: #141316;
    border: 1px solid #3a3542;
    color: #fff;
    border-radius: 0.25rem;
    cursor: pointer;
    font-size: 1em;
    text-decoration: none;
    min-width: 15rem;

    /* Background image URL */
    background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' width='18' height='18' fill='%23333'><path d='M7 10l5 5 5-5z'/></svg>");
    background-repeat: no-repeat;
    background-position: right 12px center;
    background-size: 24px;
    padding-right: 30px;
  }

  .tile-input {
    appearance: none;
    padding: 0.5rem 0.5rem;
    border: none;
    background-color: #141316;
    border: 1px solid #3a3542;
    color: #fff;
    border-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
    font-size: 0.9em;

    text-decoration: none;
    min-width: 15rem;
  }

  .tile-checkbox {
    appearance: none;
    width: 20px;
    height: 20px;
    border: 1px solid #3a3542;
    border-radius: 4px;
    background-color: #141316;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.3s ease;
    cursor: pointer;
  }

  .tile-checkbox::after {
    content: "✔";
    font-size: 14px;
    color: #007bff;
    display: none;
  }

  .tile-checkbox:checked {
    border-color: #947cc0;
    background-color: #735c9e;
  }

  .tile-checkbox:checked::after {
    display: block;
    color: white;
  }

  .tile-button {
    padding: 0.5rem;
    background-color: #544d5e;
    border: none;
    color: #fff;
    border-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
    cursor: pointer;
    font-size: 1em;
    text-decoration: none;
  }
</style>
