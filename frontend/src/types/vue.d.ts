export {}
declare module 'vue' {
  interface ComponentCustomProperties {
    $apiUrl: string
  }
}